use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Idle,
    Warmup,
    Work,
    Rest,
    InterRoundRest,
    Cooldown,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutConfig {
    pub work_duration: u32,       // default: 40s
    pub rest_duration: u32,       // default: 20s
    pub inter_round_rest: u32,    // default: 35s
    pub total_rounds: u32,        // default: 2
    pub warmup_duration: u32,     // default: 60s (0 to disable)
    pub cooldown_duration: u32,   // default: 60s (0 to disable)
    pub speech_enabled: bool,     // default: true
    pub beeps_enabled: bool,      // default: true
    pub lang: String,             // "id-ID" or "en-US"
}

impl Default for WorkoutConfig {
    fn default() -> Self {
        Self {
            work_duration: 40,
            rest_duration: 20,
            inter_round_rest: 35,
            total_rounds: 2,
            warmup_duration: 60,
            cooldown_duration: 60,
            speech_enabled: true,
            beeps_enabled: true,
            lang: "id-ID".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerSnapshot {
    pub phase: Phase,
    pub time_remaining: u32,
    pub total_time_for_phase: u32,
    pub current_round: u32,
    pub total_rounds: u32,
    pub current_move_idx: usize,
    pub total_moves: usize,
    pub current_move_name: String,
    pub next_move_name: Option<String>,
    pub is_running: bool,
    pub day_key: String,
    pub day_title: String,
    pub day_focus: String,
}

pub struct WorkoutStateMachine {
    pub config: WorkoutConfig,
    pub moves: Vec<String>,
    pub day_key: String,
    pub day_title: String,
    pub day_focus: String,

    pub phase: Phase,
    pub time_remaining: u32,
    pub current_round: u32,
    pub current_move_idx: usize,
    pub is_running: bool,
    pub halfway_notified: bool,
}

impl WorkoutStateMachine {
    pub fn new(day_key: String, day_title: String, day_focus: String, moves: Vec<String>, config: WorkoutConfig) -> Self {
        Self {
            config,
            moves,
            day_key,
            day_title,
            day_focus,
            phase: Phase::Idle,
            time_remaining: 0,
            current_round: 1,
            current_move_idx: 0,
            is_running: false,
            halfway_notified: false,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;
        self.current_round = 1;
        self.current_move_idx = 0;
        self.halfway_notified = false;

        if self.config.warmup_duration > 0 {
            self.phase = Phase::Warmup;
            self.time_remaining = self.config.warmup_duration;
        } else {
            self.start_work_interval();
        }
    }

    pub fn pause(&mut self) {
        self.is_running = false;
    }

    pub fn resume(&mut self) {
        self.is_running = true;
    }

    pub fn reset(&mut self) {
        self.is_running = false;
        self.phase = Phase::Idle;
        self.time_remaining = 0;
        self.current_round = 1;
        self.current_move_idx = 0;
        self.halfway_notified = false;
    }

    fn start_work_interval(&mut self) {
        self.phase = Phase::Work;
        self.time_remaining = self.config.work_duration;
        self.halfway_notified = false;
    }

    pub fn current_move(&self) -> String {
        if self.moves.is_empty() {
            return "Gerakan".to_string();
        }
        self.moves.get(self.current_move_idx).cloned().unwrap_or_else(|| self.moves[0].clone())
    }

    pub fn next_move(&self) -> Option<String> {
        if self.moves.is_empty() {
            return None;
        }
        if self.current_move_idx + 1 < self.moves.len() {
            Some(self.moves[self.current_move_idx + 1].clone())
        } else if self.current_round < self.config.total_rounds {
            // First move of next round
            Some(format!("(Putaran {}) {}", self.current_round + 1, self.moves[0]))
        } else if self.config.cooldown_duration > 0 {
            Some("Pendinginan (Cool Down)".to_string())
        } else {
            Some("Selesai!".to_string())
        }
    }

    // Returns an optional Action cue for audio/sound
    pub fn tick(&mut self) -> Option<AudioEvent> {
        if !self.is_running || self.phase == Phase::Idle || self.phase == Phase::Finished {
            return None;
        }

        let mut event = None;

        // Countdown beeps at 3, 2, 1 seconds before phase transition
        if self.time_remaining <= 3 && self.time_remaining > 0 {
            event = Some(AudioEvent::Countdown(self.time_remaining));
        }

        // Halfway notification during Work phase
        if self.phase == Phase::Work && !self.halfway_notified {
            let half = self.config.work_duration / 2;
            if self.time_remaining <= half {
                self.halfway_notified = true;
                event = Some(AudioEvent::Halfway);
            }
        }

        if self.time_remaining > 0 {
            self.time_remaining -= 1;
        }

        // Phase Transition Triggered
        if self.time_remaining == 0 {
            event = self.transition_phase();
        }

        event
    }

    fn transition_phase(&mut self) -> Option<AudioEvent> {
        match self.phase {
            Phase::Warmup => {
                self.start_work_interval();
                Some(AudioEvent::StartWork(self.current_move()))
            }
            Phase::Work => {
                let is_last_move_in_round = self.current_move_idx + 1 >= self.moves.len();
                if is_last_move_in_round {
                    if self.current_round >= self.config.total_rounds {
                        // Finished all moves and rounds!
                        if self.config.cooldown_duration > 0 {
                            self.phase = Phase::Cooldown;
                            self.time_remaining = self.config.cooldown_duration;
                            Some(AudioEvent::StartCooldown)
                        } else {
                            self.phase = Phase::Finished;
                            self.is_running = false;
                            Some(AudioEvent::WorkoutFinished)
                        }
                    } else {
                        // Rest between rounds
                        self.phase = Phase::InterRoundRest;
                        self.time_remaining = self.config.inter_round_rest;
                        let next = self.moves.first().cloned();
                        Some(AudioEvent::StartInterRoundRest {
                            round_completed: self.current_round,
                            next_move: next,
                        })
                    }
                } else {
                    // Normal rest between moves
                    self.phase = Phase::Rest;
                    self.time_remaining = self.config.rest_duration;
                    let next = self.next_move();
                    Some(AudioEvent::StartRest { next_move: next })
                }
            }
            Phase::Rest => {
                self.current_move_idx += 1;
                self.start_work_interval();
                Some(AudioEvent::StartWork(self.current_move()))
            }
            Phase::InterRoundRest => {
                self.current_round += 1;
                self.current_move_idx = 0;
                self.start_work_interval();
                Some(AudioEvent::StartWork(self.current_move()))
            }
            Phase::Cooldown => {
                self.phase = Phase::Finished;
                self.is_running = false;
                Some(AudioEvent::WorkoutFinished)
            }
            _ => None,
        }
    }

    pub fn snapshot(&self) -> TimerSnapshot {
        let total_for_phase = match self.phase {
            Phase::Warmup => self.config.warmup_duration,
            Phase::Work => self.config.work_duration,
            Phase::Rest => self.config.rest_duration,
            Phase::InterRoundRest => self.config.inter_round_rest,
            Phase::Cooldown => self.config.cooldown_duration,
            _ => 0,
        };

        TimerSnapshot {
            phase: self.phase,
            time_remaining: self.time_remaining,
            total_time_for_phase: total_for_phase,
            current_round: self.current_round,
            total_rounds: self.config.total_rounds,
            current_move_idx: self.current_move_idx,
            total_moves: self.moves.len(),
            current_move_name: self.current_move(),
            next_move_name: self.next_move(),
            is_running: self.is_running,
            day_key: self.day_key.clone(),
            day_title: self.day_title.clone(),
            day_focus: self.day_focus.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AudioEvent {
    Countdown(u32),
    Halfway,
    StartWork(String),
    StartRest { next_move: Option<String> },
    StartInterRoundRest { round_completed: u32, next_move: Option<String> },
    StartCooldown,
    WorkoutFinished,
}
