use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, GainNode, OscillatorNode, SpeechSynthesisUtterance};

// ponytail: direct native Web Audio & Web Speech APIs eliminate the need for heavy MP3 assets or audio fetch latency.
pub struct AudioEngine {
    audio_ctx: Option<AudioContext>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let ctx = AudioContext::new().ok();
        Self { audio_ctx: ctx }
    }

    pub fn play_tone(&self, frequency: f32, duration_secs: f64, gain_level: f32) {
        if let Some(ctx) = &self.audio_ctx {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(web_sys::OscillatorType::Sine);
                    osc.frequency().set_value(frequency);

                    // Fade in and fade out envelope to avoid speaker clicks
                    gain.gain().set_value_at_time(0.001, now).ok();
                    gain.gain().exponential_ramp_to_value_at_time(gain_level, now + 0.02).ok();
                    gain.gain().exponential_ramp_to_value_at_time(0.0001, now + duration_secs).ok();

                    if osc.connect_with_audio_node(&gain).is_ok()
                        && gain.connect_with_audio_node(&ctx.destination()).is_ok()
                    {
                        osc.start_with_when(now).ok();
                        osc.stop_with_when(now + duration_secs).ok();
                    }
                }
            }
        }
    }

    pub fn beep_countdown(&self) {
        // High pitch short warning pip (880 Hz / A5)
        self.play_tone(880.0, 0.15, 0.25);
    }

    pub fn beep_start(&self) {
        // Low to high double chime (1174.66 Hz / D6)
        self.play_tone(1174.66, 0.40, 0.35);
    }

    pub fn beep_halfway(&self) {
        // Distinctive half-time melodic pulse (659.25 Hz / E5)
        self.play_tone(659.25, 0.25, 0.30);
    }

    pub fn beep_rest(&self) {
        // Calming lower pitch tone (392.0 Hz / G4)
        self.play_tone(392.0, 0.35, 0.25);
    }

    pub fn speak(text: &str, lang: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(speech_synth) = window.speech_synthesis() {
                if let Ok(utterance) = SpeechSynthesisUtterance::new_with_text(text) {
                    utterance.set_lang(lang);
                    utterance.set_rate(1.05); // slight speedup for brisk workout tempo
                    utterance.set_pitch(1.0);
                    utterance.set_volume(1.0);
                    speech_synth.cancel(); // cancel pending speech so current cue is prompt
                    speech_synth.speak(&utterance);
                }
            }
        }
    }
}
