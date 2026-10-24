pub mod audio;
pub mod schedule;
pub mod state;

use wasm_bindgen::prelude::*;
use schedule::WorkoutSchedule;
use state::{AudioEvent, WorkoutConfig, WorkoutStateMachine};

#[wasm_bindgen]
pub struct GrindinEngine {
    schedule: WorkoutSchedule,
    state_machine: WorkoutStateMachine,
    audio: audio::AudioEngine,
}

#[wasm_bindgen]
impl GrindinEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(day_index: u8, custom_schedule_json: Option<String>, custom_config_json: Option<String>) -> Self {
        // ponytail: fallback to default schedule if custom json is absent or malformed
        let schedule = if let Some(json_str) = custom_schedule_json {
            serde_json::from_str::<WorkoutSchedule>(&json_str).unwrap_or_default()
        } else {
            WorkoutSchedule::default()
        };

        let config = if let Some(cfg_str) = custom_config_json {
            serde_json::from_str::<WorkoutConfig>(&cfg_str).unwrap_or_default()
        } else {
            WorkoutConfig::default()
        };

        let routine = schedule.get_routine_for_day_index(day_index);
        let sm = WorkoutStateMachine::new(
            routine.day_key.clone(),
            routine.day_name.clone(),
            routine.focus.clone(),
            routine.moves.clone(),
            config,
        );

        Self {
            schedule,
            state_machine: sm,
            audio: audio::AudioEngine::new(),
        }
    }

    pub fn select_day(&mut self, day_key: &str) {
        if let Some(routine) = self.schedule.get_routine_by_key(day_key) {
            let config = self.state_machine.config.clone();
            self.state_machine = WorkoutStateMachine::new(
                routine.day_key.clone(),
                routine.day_name.clone(),
                routine.focus.clone(),
                routine.moves.clone(),
                config,
            );
        }
    }

    pub fn update_routine_moves(&mut self, day_key: &str, moves_json: &str) {
        if let Ok(new_moves) = serde_json::from_str::<Vec<String>>(moves_json) {
            if let Some(routine) = self.schedule.routines.iter_mut().find(|r| r.day_key.eq_ignore_ascii_case(day_key)) {
                routine.moves = new_moves.clone();
            }
            if self.state_machine.day_key.eq_ignore_ascii_case(day_key) {
                self.state_machine.moves = new_moves;
            }
        }
    }

    pub fn update_config(&mut self, config_json: &str) {
        if let Ok(new_cfg) = serde_json::from_str::<WorkoutConfig>(config_json) {
            self.state_machine.config = new_cfg;
        }
    }

    pub fn start(&mut self) {
        self.state_machine.start();
        // Initial announce
        if self.state_machine.phase == state::Phase::Warmup {
            if self.state_machine.config.speech_enabled {
                audio::AudioEngine::speak("Pemanasan satu menit, mulai!", &self.state_machine.config.lang);
            }
            self.audio.beep_start();
        } else if self.state_machine.phase == state::Phase::Work {
            let mv = self.state_machine.current_move();
            if self.state_machine.config.speech_enabled {
                audio::AudioEngine::speak(&format!("{}, mulai!", mv), &self.state_machine.config.lang);
            }
            self.audio.beep_start();
        }
    }

    pub fn pause(&mut self) {
        self.state_machine.pause();
    }

    pub fn resume(&mut self) {
        self.state_machine.resume();
    }

    pub fn reset(&mut self) {
        self.state_machine.reset();
    }

    // Tick method called every 1 second by requestAnimationFrame delta
    pub fn tick(&mut self) {
        if let Some(event) = self.state_machine.tick() {
            self.handle_audio_event(event);
        }
    }

    fn handle_audio_event(&self, event: AudioEvent) {
        let lang = &self.state_machine.config.lang;
        let speech = self.state_machine.config.speech_enabled;
        let beeps = self.state_machine.config.beeps_enabled;

        match event {
            AudioEvent::Countdown(_) => {
                if beeps {
                    self.audio.beep_countdown();
                }
            }
            AudioEvent::Halfway => {
                if beeps {
                    self.audio.beep_halfway();
                }
                if speech {
                    audio::AudioEngine::speak("Setengah waktu!", lang);
                }
            }
            AudioEvent::StartWork(move_name) => {
                if beeps {
                    self.audio.beep_start();
                }
                if speech {
                    audio::AudioEngine::speak(&format!("{}, mulai!", move_name), lang);
                }
            }
            AudioEvent::StartRest { next_move } => {
                if beeps {
                    self.audio.beep_rest();
                }
                if speech {
                    let next_str = next_move.map(|m| format!("Berikutnya, {}", m)).unwrap_or_default();
                    let msg = format!("Istirahat. {}", next_str);
                    audio::AudioEngine::speak(&msg, lang);
                }
            }
            AudioEvent::StartInterRoundRest { round_completed, next_move } => {
                if beeps {
                    self.audio.beep_rest();
                }
                if speech {
                    let next_str = next_move.map(|m| format!("Berikutnya, {}", m)).unwrap_or_default();
                    let msg = format!("Putaran {} selesai! Istirahat panjang. {}", round_completed, next_str);
                    audio::AudioEngine::speak(&msg, lang);
                }
            }
            AudioEvent::StartCooldown => {
                if beeps {
                    self.audio.beep_start();
                }
                if speech {
                    audio::AudioEngine::speak("Pendinginan satu menit. Tarik napas dalam.", lang);
                }
            }
            AudioEvent::WorkoutFinished => {
                if beeps {
                    self.audio.beep_start();
                }
                if speech {
                    audio::AudioEngine::speak("Latihan selesai! Kerja bagus hari ini!", lang);
                }
            }
        }
    }

    pub fn get_snapshot_json(&self) -> String {
        serde_json::to_string(&self.state_machine.snapshot()).unwrap_or_default()
    }

    pub fn get_schedule_json(&self) -> String {
        serde_json::to_string(&self.schedule).unwrap_or_default()
    }

    pub fn get_config_json(&self) -> String {
        serde_json::to_string(&self.state_machine.config).unwrap_or_default()
    }
}
