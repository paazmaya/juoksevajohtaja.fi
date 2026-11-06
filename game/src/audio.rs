#[cfg(target_arch = "wasm32")]
use web_sys::{AudioContext, OscillatorType};

pub struct SoundManager {
    #[cfg(target_arch = "wasm32")]
    ctx: Option<AudioContext>,
    pub muted: bool,
}

impl SoundManager {
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

                    let _ = gain.gain().set_value_at_time(0.12, now);
                    let _ = gain
                        .gain()
                        .exponential_ramp_to_value_at_time(0.001, now + duration);

                    let _ = osc.connect_with_audio_node(&gain);
                    let _ = gain.connect_with_audio_node(&ctx.destination());

                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + duration);
                }
            }
        }
    }

    pub fn play_jump(&self) {
        #[cfg(target_arch = "wasm32")]
        self.play_tone(380.0, 0.12, OscillatorType::Square);
    }

    pub fn play_pickup(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.play_tone(587.33, 0.08, OscillatorType::Sine); // D5
        }
    }

    pub fn play_quiz_correct(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.play_tone(523.25, 0.15, OscillatorType::Triangle); // C5
            self.play_tone(659.25, 0.25, OscillatorType::Triangle); // E5
        }
    }

    pub fn play_quiz_wrong(&self) {
        #[cfg(target_arch = "wasm32")]
        self.play_tone(130.81, 0.35, OscillatorType::Sawtooth); // C3
    }

    pub fn play_obstacle_hit(&self) {
        #[cfg(target_arch = "wasm32")]
        self.play_tone(180.0, 0.18, OscillatorType::Sawtooth);
    }

    pub fn play_victory(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.play_tone(523.25, 0.2, OscillatorType::Triangle);
            self.play_tone(783.99, 0.4, OscillatorType::Triangle);
        }
    }

    pub fn play_game_over(&self) {
        #[cfg(target_arch = "wasm32")]
        self.play_tone(110.0, 0.5, OscillatorType::Sawtooth);
    }
}
