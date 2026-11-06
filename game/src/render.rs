#[cfg(target_arch = "wasm32")]
use crate::game::{Game, GameState};
#[cfg(target_arch = "wasm32")]
use crate::physics::GROUND_Y;
#[cfg(target_arch = "wasm32")]
use web_sys::CanvasRenderingContext2d;

#[cfg(target_arch = "wasm32")]
pub struct Renderer {
    pub width: f64,
    pub height: f64,
}

#[cfg(target_arch = "wasm32")]
impl Renderer {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        // Clear screen
        ctx.clear_rect(0.0, 0.0, self.width, self.height);

        // 1. Sky & Sun
        self.draw_sky(ctx);

        // 2. Parallax Background Layers
        self.draw_distant_hills(ctx, game.distance * 0.15);
        self.draw_midground_trees(ctx, game.distance * 0.45);

        // 3. Ground & Running Track
        self.draw_track(ctx, game.distance);

        // 4. Checkpoints & Gates
        self.draw_checkpoints(ctx, game);

        // 5. Pickups
        self.draw_pickups(ctx, game);

        // 6. Obstacles
        self.draw_obstacles(ctx, game);

        // 7. Host Runner
        self.draw_host(ctx, game);

        // 8. Player Runner
        self.draw_player(ctx, game);

        // 9. Speech Bubbles
        self.draw_dialogue(ctx, game);

        // 10. HUD Overlay
        self.draw_hud(ctx, game);

        // 11. Modal Screens (Start, Victory, Game Over, Quiz)
        match &game.state {
            GameState::Start => self.draw_start_screen(ctx, game),
            GameState::Quiz { feedback, .. } => {
                self.draw_quiz_overlay(ctx, game, feedback.as_ref())
            }
            GameState::Victory {
                score,
                time_seconds,
            } => self.draw_victory_screen(ctx, game, *score, *time_seconds),
            GameState::GameOver { reason } => self.draw_game_over_screen(ctx, reason),
            GameState::Running => {}
        }
    }

    fn draw_sky(&self, ctx: &CanvasRenderingContext2d) {
        // Nordic morning sky
        ctx.set_fill_style_str("#a8e6cf");
        ctx.fill_rect(0.0, 0.0, self.width, GROUND_Y);

        ctx.set_fill_style_str("#dcedc1");
        ctx.fill_rect(0.0, GROUND_Y - 120.0, self.width, 120.0);

        // Sun
        ctx.set_fill_style_str("#ffd32a");
        ctx.begin_path();
        let _ = ctx.arc(720.0, 70.0, 32.0, 0.0, std::f64::consts::PI * 2.0);
        ctx.fill();
    }

    fn draw_distant_hills(&self, ctx: &CanvasRenderingContext2d, offset: f64) {
        ctx.set_fill_style_str("#8bc5cc");
        ctx.begin_path();
        ctx.move_to(0.0, GROUND_Y);

        let step = 140.0;
        let mut x = -(offset % step) - step;
        while x < self.width + step * 2.0 {
            let hill_h = 70.0 + ((x + offset) * 0.01).sin() * 25.0;
            ctx.quadratic_curve_to(x + step * 0.5, GROUND_Y - hill_h, x + step, GROUND_Y);
            x += step;
        }
        ctx.line_to(self.width, GROUND_Y);
        ctx.close_path();
        ctx.fill();
    }

    fn draw_midground_trees(&self, ctx: &CanvasRenderingContext2d, offset: f64) {
        ctx.set_fill_style_str("#5a9e78");
        let tree_spacing = 160.0;
        let mut x = -(offset % tree_spacing) - tree_spacing;
        while x < self.width + tree_spacing * 2.0 {
            // Tree trunk
            ctx.set_fill_style_str("#7d5e3f");
            ctx.fill_rect(x + 18.0, GROUND_Y - 55.0, 10.0, 55.0);

            // Foliage
            ctx.set_fill_style_str("#458a62");
            ctx.begin_path();
            let _ = ctx.arc(
                x + 23.0,
                GROUND_Y - 75.0,
                30.0,
                0.0,
                std::f64::consts::PI * 2.0,
            );
            ctx.fill();

            x += tree_spacing;
        }
    }

    fn draw_track(&self, ctx: &CanvasRenderingContext2d, distance: f64) {
        // Ground asphalt
        ctx.set_fill_style_str("#2f3640");
        ctx.fill_rect(0.0, GROUND_Y, self.width, self.height - GROUND_Y);

        // Grass verge
        ctx.set_fill_style_str("#44bd32");
        ctx.fill_rect(0.0, GROUND_Y, self.width, 10.0);

        // Running track lane lines & distance markers
        ctx.set_fill_style_str("#f5f6fa");
        let marker_spacing = 100.0; // every 100 pixels
        let mut marker_x = -(distance % marker_spacing);
        while marker_x < self.width + marker_spacing {
            ctx.fill_rect(marker_x, GROUND_Y + 35.0, 40.0, 4.0);
            marker_x += marker_spacing;
        }

        // Distance numbers along the pavement
        ctx.set_fill_style_str("#718093");
        ctx.set_font("bold 12px sans-serif");
        let meter_spacing = 250.0;
        let start_meter = (distance / meter_spacing).floor() * meter_spacing;
        for i in 0..6 {
            let m = start_meter + (i as f64) * meter_spacing;
            let screen_x = m - distance + 120.0;
            if screen_x >= 0.0 && screen_x <= self.width {
                let _ = ctx.fill_text(&format!("{:.0}m", m), screen_x, GROUND_Y + 65.0);
            }
        }
    }

    fn draw_checkpoints(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        for (_, cp) in &game.active_checkpoints {
            let screen_x = cp.distance - game.distance + game.player.x;
            if screen_x > -150.0 && screen_x < self.width + 150.0 {
                // Checkpoint Arch / Gate
                ctx.set_fill_style_str("#e67e22");
                ctx.fill_rect(screen_x - 12.0, GROUND_Y - 180.0, 14.0, 180.0);
                ctx.fill_rect(screen_x + 90.0, GROUND_Y - 180.0, 14.0, 180.0);
                // Arch top banner
                ctx.fill_rect(screen_x - 16.0, GROUND_Y - 210.0, 124.0, 35.0);

                ctx.set_fill_style_str("#ffffff");
                ctx.set_font("bold 14px sans-serif");
                let _ = ctx.fill_text("🎙️ KYSYMYS", screen_x - 6.0, GROUND_Y - 187.0);

                // Hanging sign
                ctx.set_fill_style_str("#f39c12");
                ctx.fill_rect(screen_x + 5.0, GROUND_Y - 165.0, 80.0, 45.0);
                ctx.set_fill_style_str("#2c3e50");
                ctx.set_font("bold 11px sans-serif");
                let _ = ctx.fill_text("JAKSO-OPPI", screen_x + 12.0, GROUND_Y - 138.0);
            }
        }
    }

    fn draw_pickups(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        for pickup in &game.active_pickups {
            let screen_x = pickup.distance - game.distance + game.player.x;
            if screen_x > -50.0 && screen_x < self.width + 50.0 {
                let y = GROUND_Y - 45.0;
                if pickup.kind == "coffee" {
                    // Takeaway coffee cup
                    ctx.set_fill_style_str("#e74c3c");
                    ctx.fill_rect(screen_x, y + 8.0, 24.0, 24.0);
                    ctx.set_fill_style_str("#ecf0f1");
                    ctx.fill_rect(screen_x - 2.0, y + 4.0, 28.0, 6.0);
                    ctx.set_fill_style_str("#ffffff");
                    ctx.set_font("bold 16px sans-serif");
                    let _ = ctx.fill_text("☕", screen_x + 2.0, y + 26.0);
                } else {
                    // Podcast mic
                    ctx.set_fill_style_str("#9b59b6");
                    ctx.begin_path();
                    let _ = ctx.arc(
                        screen_x + 14.0,
                        y + 14.0,
                        16.0,
                        0.0,
                        std::f64::consts::PI * 2.0,
                    );
                    ctx.fill();
                    ctx.set_fill_style_str("#ffffff");
                    ctx.set_font("bold 16px sans-serif");
                    let _ = ctx.fill_text("🎙️", screen_x + 4.0, y + 20.0);
                }
            }
        }
    }

    fn draw_obstacles(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        for obs in &game.active_obstacles {
            let screen_x = obs.distance - game.distance + game.player.x;
            if screen_x > -100.0 && screen_x < self.width + 100.0 {
                let y = GROUND_Y - obs.height;
                if obs.kind == "hurdle" {
                    // Red and white runner hurdle
                    ctx.set_fill_style_str("#c0392b");
                    ctx.fill_rect(screen_x, y, obs.width, 10.0);
                    ctx.set_fill_style_str("#ecf0f1");
                    ctx.fill_rect(screen_x + 10.0, y, 10.0, 10.0);
                    ctx.fill_rect(screen_x + 30.0, y, 10.0, 10.0);
                    ctx.set_fill_style_str("#7f8c8d");
                    ctx.fill_rect(screen_x + 4.0, y + 10.0, 5.0, obs.height - 10.0);
                    ctx.fill_rect(screen_x + obs.width - 9.0, y + 10.0, 5.0, obs.height - 10.0);
                } else if obs.kind == "meeting" {
                    // Urgent meeting box
                    ctx.set_fill_style_str("#d63031");
                    ctx.fill_rect(screen_x, y, obs.width, obs.height);
                    ctx.set_fill_style_str("#ffffff");
                    ctx.set_font("bold 10px sans-serif");
                    let _ = ctx.fill_text("MEETING", screen_x + 2.0, y + 18.0);
                    let _ = ctx.fill_text("URGENT!", screen_x + 2.0, y + 34.0);
                } else {
                    // Cactus / Bug box
                    ctx.set_fill_style_str("#27ae60");
                    ctx.fill_rect(screen_x + 12.0, y, 16.0, obs.height);
                    ctx.fill_rect(screen_x, y + 15.0, obs.width, 10.0);
                    ctx.fill_rect(screen_x, y + 8.0, 10.0, 12.0);
                    ctx.fill_rect(screen_x + obs.width - 10.0, y + 4.0, 10.0, 16.0);
                }
            }
        }
    }

    fn draw_player(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        let p = &game.player;
        let blink =
            p.invulnerability_timer > 0.0 && ((p.invulnerability_timer * 10.0) as i32 % 2 == 0);
        if blink {
            return;
        }

        // Draw Player Runner (Guest)
        let frame = p.animation_frame;
        let leg_offset = if p.is_grounded {
            if frame == 0 {
                8.0
            } else {
                -8.0
            }
        } else {
            -4.0
        };

        // Legs
        ctx.set_fill_style_str("#2c3e50"); // Dark running tights
        ctx.fill_rect(p.x + 8.0 + leg_offset, p.y + 44.0, 8.0, 26.0);
        ctx.fill_rect(p.x + 20.0 - leg_offset, p.y + 44.0, 8.0, 26.0);
        // Running shoes
        ctx.set_fill_style_str("#00cec9");
        ctx.fill_rect(p.x + 8.0 + leg_offset, p.y + 64.0, 14.0, 6.0);
        ctx.fill_rect(p.x + 20.0 - leg_offset, p.y + 64.0, 14.0, 6.0);

        // Body / Shirt (Teal athletic windbreaker)
        ctx.set_fill_style_str("#0984e3");
        ctx.fill_rect(p.x + 6.0, p.y + 20.0, 26.0, 26.0);

        // Head
        ctx.set_fill_style_str("#ffeaa7");
        ctx.begin_path();
        let _ = ctx.arc(
            p.x + 20.0,
            p.y + 12.0,
            12.0,
            0.0,
            std::f64::consts::PI * 2.0,
        );
        ctx.fill();

        // Hair / Cap
        ctx.set_fill_style_str("#2d3436");
        ctx.fill_rect(p.x + 10.0, p.y + 2.0, 18.0, 8.0);

        // Label tag
        ctx.set_fill_style_str("#2d3436");
        ctx.set_font("bold 11px sans-serif");
        let _ = ctx.fill_text("Sinä (Vieras)", p.x - 10.0, p.y - 6.0);
    }

    fn draw_host(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        let h = &game.host;
        let host_x = game.player.x + h.x_offset;
        let host_y = h.y;

        let frame = h.animation_frame;
        let leg_offset = if h.is_grounded {
            if frame == 0 {
                8.0
            } else {
                -8.0
            }
        } else {
            -4.0
        };

        // Legs
        ctx.set_fill_style_str("#1e272e");
        ctx.fill_rect(host_x + 8.0 + leg_offset, host_y + 44.0, 8.0, 26.0);
        ctx.fill_rect(host_x + 20.0 - leg_offset, host_y + 44.0, 8.0, 26.0);
        // Bright orange running shoes
        ctx.set_fill_style_str("#ff3f34");
        ctx.fill_rect(host_x + 8.0 + leg_offset, host_y + 64.0, 14.0, 6.0);
        ctx.fill_rect(host_x + 20.0 - leg_offset, host_y + 64.0, 14.0, 6.0);

        // Host shirt: Signature Safety Neon Orange
        ctx.set_fill_style_str("#ff5e57");
        ctx.fill_rect(host_x + 6.0, host_y + 20.0, 26.0, 26.0);

        // Podcast microphone badge on shirt
        ctx.set_fill_style_str("#ffffff");
        ctx.fill_rect(host_x + 14.0, host_y + 26.0, 8.0, 10.0);

        // Head
        ctx.set_fill_style_str("#f8c291");
        ctx.begin_path();
        let _ = ctx.arc(
            host_x + 20.0,
            host_y + 12.0,
            12.0,
            0.0,
            std::f64::consts::PI * 2.0,
        );
        ctx.fill();

        // Headphones
        ctx.set_fill_style_str("#3c40c6");
        ctx.fill_rect(host_x + 8.0, host_y + 6.0, 4.0, 12.0);
        ctx.fill_rect(host_x + 28.0, host_y + 6.0, 4.0, 12.0);
        ctx.fill_rect(host_x + 10.0, host_y + 2.0, 20.0, 4.0);

        // Label tag
        ctx.set_fill_style_str("#d63031");
        ctx.set_font("bold 11px sans-serif");
        let _ = ctx.fill_text("Jukka (Juokseva Johtaja)", host_x - 30.0, host_y - 6.0);
    }

    fn draw_dialogue(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        if let Some(msg) = &game.host.active_dialogue {
            let host_x = game.player.x + game.host.x_offset;
            let bubble_x = host_x - 30.0;
            let bubble_y = game.host.y - 65.0;

            ctx.set_font("bold 12px sans-serif");
            let bubble_w = ((msg.len() as f64) * 7.5 + 24.0).max(120.0);
            let bubble_h = 36.0;

            // Speech bubble background
            ctx.set_fill_style_str("#ffffff");
            ctx.set_stroke_style_str("#2f3542");
            ctx.set_line_width(2.0);
            ctx.fill_rect(bubble_x, bubble_y, bubble_w, bubble_h);
            ctx.stroke_rect(bubble_x, bubble_y, bubble_w, bubble_h);

            // Speech text
            ctx.set_fill_style_str("#2f3542");
            let _ = ctx.fill_text(msg, bubble_x + 10.0, bubble_y + 22.0);
        }
    }

    fn draw_hud(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        // HUD container top bar
        ctx.set_fill_style_str("rgba(47, 53, 66, 0.85)");
        ctx.fill_rect(0.0, 0.0, self.width, 48.0);

        // Episode Title
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 15px sans-serif");
        let _ = ctx.fill_text(&game.level.title, 20.0, 28.0);

        // Distance & Progress
        let pct = game.progress_percent();
        ctx.set_font("14px sans-serif");
        let dist_text = format!(
            "{:.0}m / {:.0}m ({:.0}%)",
            game.distance, game.level.track_length, pct
        );
        let _ = ctx.fill_text(&dist_text, 360.0, 28.0);

        // Stamina Bar
        ctx.set_fill_style_str("#ffffff");
        let _ = ctx.fill_text("Kunto:", 580.0, 28.0);
        ctx.set_fill_style_str("#57606f");
        ctx.fill_rect(635.0, 14.0, 140.0, 18.0);

        let stamina_pct = (game.player.stamina / game.player.max_stamina).clamp(0.0, 1.0);
        let bar_color = if stamina_pct > 0.5 {
            "#2ed573"
        } else if stamina_pct > 0.25 {
            "#ffa502"
        } else {
            "#ff4757"
        };
        ctx.set_fill_style_str(bar_color);
        ctx.fill_rect(637.0, 16.0, 136.0 * stamina_pct, 14.0);

        // Score
        ctx.set_fill_style_str("#ffd32a");
        ctx.set_font("bold 15px sans-serif");
        let _ = ctx.fill_text(&format!("Pisteet: {}", game.player.score), 800.0, 28.0);
    }

    fn draw_start_screen(&self, ctx: &CanvasRenderingContext2d, game: &Game) {
        ctx.set_fill_style_str("rgba(0, 0, 0, 0.65)");
        ctx.fill_rect(0.0, 0.0, self.width, self.height);

        let center_x = self.width / 2.0;

        ctx.set_fill_style_str("#ffd32a");
        ctx.set_font("bold 38px sans-serif");
        let _ = ctx.fill_text("JUOKSEVA JOHTAJA", center_x - 220.0, 180.0);

        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 20px sans-serif");
        let _ = ctx.fill_text(&game.level.title, center_x - 180.0, 225.0);

        ctx.set_font("16px sans-serif");
        let _ = ctx.fill_text(
            "Pelaat haastateltavana juoksemassa podcastin isännän Jukka Paasosen kanssa.",
            center_x - 280.0,
            270.0,
        );
        let _ = ctx.fill_text(
            "Vastaa podcast-jakson oppeja mittaaviin kysymyksiin ja vältä esteitä!",
            center_x - 260.0,
            300.0,
        );

        ctx.set_fill_style_str("#2ed573");
        ctx.fill_rect(center_x - 170.0, 340.0, 340.0, 60.0);
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 22px sans-serif");
        let _ = ctx.fill_text("ALOTA LENKKI [VÄLILYÖNTI]", center_x - 145.0, 378.0);
    }

    fn draw_quiz_overlay(
        &self,
        ctx: &CanvasRenderingContext2d,
        game: &Game,
        feedback: Option<&crate::game::QuizFeedback>,
    ) {
        ctx.set_fill_style_str("rgba(0, 0, 0, 0.75)");
        ctx.fill_rect(0.0, 0.0, self.width, self.height);

        let cp = match game.current_checkpoint() {
            Some(c) => c,
            None => return,
        };

        let card_w = 720.0;
        let card_h = 360.0;
        let card_x = (self.width - card_w) / 2.0;
        let card_y = 70.0;

        // Question card
        ctx.set_fill_style_str("#ffffff");
        ctx.fill_rect(card_x, card_y, card_w, card_h);
        ctx.set_stroke_style_str("#e67e22");
        ctx.set_line_width(4.0);
        ctx.stroke_rect(card_x, card_y, card_w, card_h);

        // Header
        ctx.set_fill_style_str("#e67e22");
        ctx.fill_rect(card_x, card_y, card_w, 48.0);
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 18px sans-serif");
        let _ = ctx.fill_text("🎙️ PODCAST-KYSYMYS RASTILLA", card_x + 20.0, card_y + 32.0);

        // Question text
        ctx.set_fill_style_str("#2c3e50");
        ctx.set_font("bold 18px sans-serif");
        let _ = ctx.fill_text(&cp.question, card_x + 25.0, card_y + 90.0);

        if let Some(ts) = &cp.podcast_timestamp {
            ctx.set_fill_style_str("#7f8c8d");
            ctx.set_font("italic 14px sans-serif");
            let _ = ctx.fill_text(
                &format!("(Kuuntele jaksosta kohdasta ~{})", ts),
                card_x + 25.0,
                card_y + 115.0,
            );
        }

        // Choices
        let mut btn_y = card_y + 145.0;
        for (i, opt) in cp.options.iter().enumerate() {
            let key_label = if i == 0 { "[1]" } else { "[2]" };
            ctx.set_fill_style_str("#f1f2f6");
            ctx.fill_rect(card_x + 25.0, btn_y, card_w - 50.0, 50.0);
            ctx.set_stroke_style_str("#ced6e0");
            ctx.set_line_width(2.0);
            ctx.stroke_rect(card_x + 25.0, btn_y, card_w - 50.0, 50.0);

            ctx.set_fill_style_str("#2f3542");
            ctx.set_font("bold 16px sans-serif");
            let _ = ctx.fill_text(
                &format!("{} {}", key_label, opt.text),
                card_x + 40.0,
                btn_y + 32.0,
            );

            btn_y += 62.0;
        }

        // Feedback notification if answered
        if let Some(fb) = feedback {
            let fb_bg = if fb.is_correct { "#2ed573" } else { "#ff4757" };
            ctx.set_fill_style_str(fb_bg);
            ctx.fill_rect(card_x + 25.0, card_y + 280.0, card_w - 50.0, 60.0);

            ctx.set_fill_style_str("#ffffff");
            ctx.set_font("bold 15px sans-serif");
            let header = if fb.is_correct {
                "OIKEIN! (+150 pistettä ja kuntoa)"
            } else {
                "VÄÄRIN! (-20 kuntoa)"
            };
            let _ = ctx.fill_text(header, card_x + 40.0, card_y + 305.0);
            ctx.set_font("13px sans-serif");
            let _ = ctx.fill_text(&fb.explanation, card_x + 40.0, card_y + 328.0);
        }
    }

    fn draw_victory_screen(
        &self,
        ctx: &CanvasRenderingContext2d,
        _game: &Game,
        score: u32,
        time_sec: f64,
    ) {
        ctx.set_fill_style_str("rgba(0, 0, 0, 0.85)");
        ctx.fill_rect(0.0, 0.0, self.width, self.height);

        let center_x = self.width / 2.0;

        ctx.set_fill_style_str("#ffd32a");
        ctx.set_font("bold 42px sans-serif");
        let _ = ctx.fill_text("🏁 MAALI SAAVUTETTU!", center_x - 240.0, 160.0);

        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 22px sans-serif");
        let _ = ctx.fill_text(
            "Onneksi olkoon, selvitsit jakson lenkin!",
            center_x - 190.0,
            210.0,
        );

        ctx.set_font("18px sans-serif");
        let _ = ctx.fill_text(
            &format!("Loppupisteet: {} pistettä", score),
            center_x - 130.0,
            255.0,
        );
        let _ = ctx.fill_text(
            &format!("Aika: {:.1} sekuntia", time_sec),
            center_x - 110.0,
            285.0,
        );

        ctx.set_fill_style_str("#2ed573");
        ctx.fill_rect(center_x - 160.0, 330.0, 320.0, 55.0);
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 20px sans-serif");
        let _ = ctx.fill_text("PELAA UUDELLEEN [R]", center_x - 120.0, 365.0);
    }

    fn draw_game_over_screen(&self, ctx: &CanvasRenderingContext2d, reason: &str) {
        ctx.set_fill_style_str("rgba(0, 0, 0, 0.85)");
        ctx.fill_rect(0.0, 0.0, self.width, self.height);

        let center_x = self.width / 2.0;

        ctx.set_fill_style_str("#ff4757");
        ctx.set_font("bold 42px sans-serif");
        let _ = ctx.fill_text("LENKKI KESKEYTYI", center_x - 190.0, 180.0);

        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("18px sans-serif");
        let _ = ctx.fill_text(reason, center_x - 220.0, 230.0);

        ctx.set_fill_style_str("#ffa502");
        ctx.fill_rect(center_x - 150.0, 290.0, 300.0, 55.0);
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 20px sans-serif");
        let _ = ctx.fill_text("YRITÄ UUDELLEEN [R]", center_x - 110.0, 325.0);
    }
}
