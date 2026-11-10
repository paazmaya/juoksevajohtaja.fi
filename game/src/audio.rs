//! Web Audio synthesis for retro sound effects (jumps, pickups, quizzes, collisions).

#[cfg(target_arch = "wasm32")]
use web_sys::{AudioContext, OscillatorType};

/// Initial gain amplitude for synthesized sound effects.
pub const AUDIO_INITIAL_GAIN: f32 = 0.12;

/// Minimum exponential ramp target gain (close to zero).
pub const AUDIO_MIN_GAIN: f32 = 0.001;

/// Jump sound oscillator frequency in Hz.
pub const JUMP_TONE_FREQ: f32 = 380.0;
/// Jump sound duration in seconds.
pub const JUMP_TONE_DURATION: f64 = 0.12;

/// Pickup sound oscillator frequency in Hz (musical D5).
pub const PICKUP_TONE_FREQ: f32 = 587.33;
/// Pickup sound duration in seconds.
pub const PICKUP_TONE_DURATION: f64 = 0.08;

/// Quiz correct answer first tone frequency in Hz (musical C5).
pub const QUIZ_CORRECT_TONE_1_FREQ: f32 = 523.25;
/// Quiz correct answer first tone duration in seconds.
pub const QUIZ_CORRECT_TONE_1_DURATION: f64 = 0.15;
/// Quiz correct answer second tone frequency in Hz (musical E5).
pub const QUIZ_CORRECT_TONE_2_FREQ: f32 = 659.25;
/// Quiz correct answer second tone duration in seconds.
pub const QUIZ_CORRECT_TONE_2_DURATION: f64 = 0.25;

/// Quiz wrong answer tone frequency in Hz (low buzzing C3).
pub const QUIZ_WRONG_TONE_FREQ: f32 = 130.81;
/// Quiz wrong answer tone duration in seconds.
pub const QUIZ_WRONG_TONE_DURATION: f64 = 0.35;

/// Obstacle collision impact tone frequency in Hz.
pub const OBSTACLE_HIT_TONE_FREQ: f32 = 180.0;
/// Obstacle collision impact tone duration in seconds.
pub const OBSTACLE_HIT_TONE_DURATION: f64 = 0.18;

/// Victory fanfare first tone frequency in Hz.
pub const VICTORY_TONE_1_FREQ: f32 = 523.25;
/// Victory fanfare first tone duration in seconds.
pub const VICTORY_TONE_1_DURATION: f64 = 0.2;
/// Victory fanfare second tone frequency in Hz.
pub const VICTORY_TONE_2_FREQ: f32 = 783.99;
/// Victory fanfare second tone duration in seconds.
pub const VICTORY_TONE_2_DURATION: f64 = 0.4;

/// Game over defeat tone frequency in Hz.
pub const GAME_OVER_TONE_FREQ: f32 = 110.0;
/// Game over defeat tone duration in seconds.
pub const GAME_OVER_TONE_DURATION: f64 = 0.5;

/// Web Audio API sound synthesizer managing procedural sound effects.
pub struct SoundManager {
    #[cfg(target_arch = "wasm32")]
    ctx: Option<AudioContext>,
    /// Whether audio playback is currently muted.
    pub muted: bool,
}

impl Default for SoundManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundManager {
    /// Creates a new `SoundManager`, initializing the browser's Web Audio context if available.
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        let ctx = AudioContext::new().ok();
        Self {
            #[cfg(target_arch = "wasm32")]
            ctx,
            muted: false,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn ensure_active(&self) {
        if let Some(ctx) = &self.ctx {
            if ctx.state() == web_sys::AudioContextState::Suspended {
                let _ = ctx.resume();
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn play_tone(&self, freq: f32, duration: f64, osc_type: OscillatorType) {
        if self.muted {
            return;
        }
        self.ensure_active();
        if let Some(ctx) = &self.ctx {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(gain) = ctx.create_gain() {
                    let now = ctx.current_time();
                    osc.set_type(osc_type);
                    osc.frequency().set_value(freq);

                    let _ = gain.gain().set_value_at_time(AUDIO_INITIAL_GAIN, now);
                    let _ = gain
                        .gain()
                        .exponential_ramp_to_value_at_time(AUDIO_MIN_GAIN, now + duration);

                    let _ = osc.connect_with_audio_node(&gain);
                    let _ = gain.connect_with_audio_node(&ctx.destination());

                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + duration);
                }
            }
        }
    }

    /// Plays a jump sound effect.
    pub fn play_jump(&self) {
        #[cfg(target_arch = "wasm32")]
        self.play_tone(JUMP_TONE_FREQ, JUMP_TONE_DURATION, OscillatorType::Square);
    }

    /// Plays a pickup collected sound effect.
    pub fn play_pickup(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.play_tone(PICKUP_TONE_FREQ, PICKUP_TONE_DURATION, OscillatorType::Sine);
        }
    }

    /// Plays a two-tone chime for a correct quiz answer.
    pub fn play_quiz_correct(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.play_tone(
                QUIZ_CORRECT_TONE_1_FREQ,
                QUIZ_CORRECT_TONE_1_DURATION,
                OscillatorType::Triangle,
            );
            self.play_tone(
                QUIZ_CORRECT_TONE_2_FREQ,
                QUIZ_CORRECT_TONE_2_DURATION,
                OscillatorType::Triangle,
            );
        }
    }

    /// Plays a low buzzer sound for an incorrect quiz answer.
    pub fn play_quiz_wrong(&self) {
        #[cfg(target_arch = "wasm32")]
        self.play_tone(
            QUIZ_WRONG_TONE_FREQ,
            QUIZ_WRONG_TONE_DURATION,
            OscillatorType::Sawtooth,
        );
    }

    /// Plays a crunch sound effect when colliding with an obstacle.
    pub fn play_obstacle_hit(&self) {
        #[cfg(target_arch = "wasm32")]
        self.play_tone(
            OBSTACLE_HIT_TONE_FREQ,
            OBSTACLE_HIT_TONE_DURATION,
            OscillatorType::Sawtooth,
        );
    }

    /// Plays a celebratory victory fanfare upon reaching the finish line.
    pub fn play_victory(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.play_tone(
                VICTORY_TONE_1_FREQ,
                VICTORY_TONE_1_DURATION,
                OscillatorType::Triangle,
            );
            self.play_tone(
                VICTORY_TONE_2_FREQ,
                VICTORY_TONE_2_DURATION,
                OscillatorType::Triangle,
            );
        }
    }

    /// Plays a defeat sound when runner stamina is completely exhausted.
    pub fn play_game_over(&self) {
        #[cfg(target_arch = "wasm32")]
        self.play_tone(
            GAME_OVER_TONE_FREQ,
            GAME_OVER_TONE_DURATION,
            OscillatorType::Sawtooth,
        );
    }
}
