//! 2D HTML5 Canvas rendering for running tracks, parallax backgrounds, runners, and HUD.

use crate::canvas::Canvas;
use crate::game::{Game, GameState};
#[cfg(target_arch = "wasm32")]
use crate::level::Level;
use crate::physics::{GROUND_Y, RUNNER_HEIGHT, RUNNER_WIDTH};
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlImageElement;

/// Parallax scroll factor for the sky layer.
pub const PARALLAX_SKY_SPEED: f64 = 0.05;

/// Lower band height of the sky gradient in pixels.
pub const SKY_LOWER_BAND_HEIGHT: f64 = 120.0;
/// Horizontal center coordinate of the sun in pixels.
pub const SUN_CENTER_X: f64 = 720.0;
/// Vertical center coordinate of the sun in pixels.
pub const SUN_CENTER_Y: f64 = 70.0;
/// Radius of the sun circle in pixels.
pub const SUN_RADIUS: f64 = 32.0;

/// Parallax scroll factor for distant hills.
pub const PARALLAX_HILLS_SPEED: f64 = 0.15;
/// Horizontal step size between hill wave points in pixels.
pub const HILLS_STEP: f64 = 140.0;
/// Baseline height of background hills in pixels.
pub const HILLS_BASE_HEIGHT: f64 = 70.0;
/// Sine wave amplitude of background hills in pixels.
pub const HILLS_AMPLITUDE: f64 = 25.0;
/// Sine wave frequency multiplier for hills.
pub const HILLS_FREQUENCY: f64 = 0.01;

/// Parallax scroll factor for midground trees.
pub const PARALLAX_TREES_SPEED: f64 = 0.45;
/// Horizontal spacing between midground trees in pixels.
pub const TREE_SPACING: f64 = 160.0;

/// Height of the green grass verge above the running track in pixels.
pub const GRASS_VERGE_HEIGHT: f64 = 10.0;
/// Spacing between white lane dashes on the track in pixels.
pub const LANE_MARKER_SPACING: f64 = 100.0;
/// Vertical offset of lane dashes below the ground line in pixels.
pub const LANE_MARKER_Y_OFFSET: f64 = 35.0;
/// Width of a lane dash in pixels.
pub const LANE_MARKER_WIDTH: f64 = 40.0;
/// Height of a lane dash in pixels.
pub const LANE_MARKER_HEIGHT: f64 = 4.0;
/// Spacing between distance labels on the track in meters/pixels.
pub const DISTANCE_METER_SPACING: f64 = 250.0;
/// Vertical offset of distance labels below the ground line in pixels.
pub const DISTANCE_METER_Y_OFFSET: f64 = 65.0;
/// Number of upcoming distance labels to draw on screen.
pub const DISTANCE_METER_LABEL_COUNT: usize = 6;

/// Height of the top in-game HUD bar in pixels.
pub const HUD_BAR_HEIGHT: f64 = 48.0;
/// Horizontal coordinate of the stamina bar container in pixels.
pub const STAMINA_BAR_BG_X: f64 = 635.0;
/// Vertical coordinate of the stamina bar container in pixels.
pub const STAMINA_BAR_BG_Y: f64 = 14.0;
/// Width of the stamina bar container in pixels.
pub const STAMINA_BAR_BG_WIDTH: f64 = 140.0;
/// Height of the stamina bar container in pixels.
pub const STAMINA_BAR_BG_HEIGHT: f64 = 18.0;
/// Horizontal coordinate of the inner stamina fill bar in pixels.
pub const STAMINA_BAR_FILL_X: f64 = 637.0;
/// Vertical coordinate of the inner stamina fill bar in pixels.
pub const STAMINA_BAR_FILL_Y: f64 = 16.0;
/// Maximum width of the inner stamina fill bar in pixels.
pub const STAMINA_BAR_FILL_MAX_WIDTH: f64 = 136.0;
/// Height of the inner stamina fill bar in pixels.
pub const STAMINA_BAR_FILL_HEIGHT: f64 = 14.0;
/// Threshold ratio above which stamina is rendered green.
pub const STAMINA_HEALTHY_RATIO: f64 = 0.5;
/// Threshold ratio above which stamina is rendered yellow/orange.
pub const STAMINA_WARNING_RATIO: f64 = 0.25;

/// Horizontal padding for culling checkpoints offscreen.
pub const CHECKPOINT_CULL_PADDING: f64 = 150.0;
/// Horizontal padding for culling pickups offscreen.
pub const PICKUP_CULL_PADDING: f64 = 50.0;
/// Horizontal padding for culling obstacles offscreen.
pub const OBSTACLE_CULL_PADDING: f64 = 100.0;

/// Horizontal position of the checkpoint question speech bubble in pixels.
pub const QUIZ_QUESTION_BOX_X: f64 = 50.0;
/// Vertical position of the checkpoint question speech bubble in pixels.
pub const QUIZ_QUESTION_BOX_Y: f64 = 60.0;
/// Width of the checkpoint question speech bubble in pixels.
pub const QUIZ_QUESTION_BOX_WIDTH: f64 = 540.0;
/// Height of the checkpoint question speech bubble in pixels.
pub const QUIZ_QUESTION_BOX_HEIGHT: f64 = 135.0;
/// Maximum characters per line for wrapping question text.
pub const QUIZ_QUESTION_MAX_CHARS: usize = 46;

/// Horizontal position of the choice speech bubbles in pixels.
pub const QUIZ_CHOICE_BOX_X: f64 = 300.0;
/// Width of the choice speech bubbles in pixels.
pub const QUIZ_CHOICE_BOX_WIDTH: f64 = 580.0;
/// Height of each choice speech bubble in pixels.
pub const QUIZ_CHOICE_BOX_HEIGHT: f64 = 66.0;
/// Vertical position of Choice A in pixels.
pub const QUIZ_CHOICE_A_Y: f64 = 220.0;
/// Vertical position of Choice B in pixels.
pub const QUIZ_CHOICE_B_Y: f64 = 300.0;
/// Maximum characters per line for wrapping choice text.
pub const QUIZ_CHOICE_MAX_CHARS: usize = 50;

/// Vertical position of the quiz feedback banner in pixels.
pub const QUIZ_FEEDBACK_BOX_Y: f64 = 380.0;
/// Height of the quiz feedback banner in pixels.
pub const QUIZ_FEEDBACK_BOX_HEIGHT: f64 = 48.0;

/// Pointer click margin tolerance around choice speech boxes in pixels.
pub const QUIZ_CLICK_TOLERANCE: f64 = 10.0;

/// Loaded images for 3-layer parallax backgrounds, obstacles, and character animations.
/// `I` is the canvas image type (`HtmlImageElement` in the browser).
pub struct ImageAssets<I> {
    /// Background Layer 1: Sky, clouds, sun.
    pub bg_layer1: Option<I>,
    /// Background Layer 2: Distant midground hills, skyline.
    pub bg_layer2: Option<I>,
    /// Background Layer 3: Near foreground scenery, roadside trees.
    pub bg_layer3: Option<I>,
    /// Obstacle: Runner hurdle.
    pub obstacle_hurdle: Option<I>,
    /// Obstacle: Urgent meeting alert box.
    pub obstacle_meeting: Option<I>,
    /// Obstacle: Cactus / bug platform.
    pub obstacle_cactus: Option<I>,
    /// Player: Idle pose.
    pub player_idle: Option<I>,
    /// Player: Walk cycle frame 1.
    pub player_walk1: Option<I>,
    /// Player: Walk cycle frame 2.
    pub player_walk2: Option<I>,
    /// Player: Jump pose.
    pub player_jump: Option<I>,
    /// Player: Victory cheer pose.
    pub player_victory: Option<I>,
    /// Host: Idle pose.
    pub host_idle: Option<I>,
    /// Host: Walk cycle frame 1.
    pub host_walk1: Option<I>,
    /// Host: Walk cycle frame 2.
    pub host_walk2: Option<I>,
    /// Host: Jump pose.
    pub host_jump: Option<I>,
    /// Host: Victory cheer pose.
    pub host_victory: Option<I>,
}

#[cfg(target_arch = "wasm32")]
fn load_image(src: &str) -> Option<HtmlImageElement> {
    let img = HtmlImageElement::new().ok()?;
    img.set_src(src);
    Some(img)
}

fn is_image_ready<'a, C: Canvas>(ctx: &C, img: &'a Option<C::Image>) -> Option<&'a C::Image> {
    img.as_ref().filter(|image| ctx.image_size(image).is_some())
}

fn draw_tiled_layer<C: Canvas>(
    ctx: &C,
    img: &C::Image,
    screen_w: f64,
    screen_h: f64,
    offset_x: f64,
) {
    let Some((img_w, img_h)) = ctx.image_size(img) else {
        return;
    };
    if img_w <= 0.0 || img_h <= 0.0 {
        return;
    }
    let scale = screen_h / img_h;
    let draw_w = img_w * scale;
    let draw_h = screen_h;
    let shift = (offset_x % draw_w + draw_w) % draw_w;
    let mut x = -shift;
    while x < screen_w {
        let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(img, x, 0.0, draw_w, draw_h);
        x += draw_w;
    }
}

#[cfg(target_arch = "wasm32")]
impl ImageAssets<HtmlImageElement> {
    /// Asynchronously requests image elements from level configuration and standard asset folders.
    pub fn load(level: Option<&Level>) -> Self {
        let (l1, l2, l3) = if let Some(bg) = level.and_then(|l| l.background.as_ref()) {
            (bg.layer1.as_str(), bg.layer2.as_str(), bg.layer3.as_str())
        } else {
            (
                "assets/backgrounds/helsinki/layer1_sky.png",
                "assets/backgrounds/helsinki/layer2_midground.png",
                "assets/backgrounds/helsinki/layer3_foreground.png",
            )
        };

        Self {
            bg_layer1: load_image(l1),
            bg_layer2: load_image(l2),
            bg_layer3: load_image(l3),
            obstacle_hurdle: load_image("assets/obstacles/obstacle_hurdle.png"),
            obstacle_meeting: load_image("assets/obstacles/obstacle_meeting.png"),
            obstacle_cactus: load_image("assets/obstacles/obstacle_cactus.png"),
            player_idle: load_image("assets/characters/adventurer/idle.png"),
            player_walk1: load_image("assets/characters/adventurer/walk1.png"),
            player_walk2: load_image("assets/characters/adventurer/walk2.png"),
            player_jump: load_image("assets/characters/adventurer/jump.png"),
            player_victory: load_image("assets/characters/adventurer/victory.png"),
            host_idle: load_image("assets/characters/player/idle.png"),
            host_walk1: load_image("assets/characters/player/walk1.png"),
            host_walk2: load_image("assets/characters/player/walk2.png"),
            host_jump: load_image("assets/characters/player/jump.png"),
            host_victory: load_image("assets/characters/player/victory.png"),
        }
    }
}

/// Canvas 2D renderer for all game visuals.
pub struct Renderer<I> {
    /// Canvas viewport width in pixels.
    pub width: f64,
    /// Canvas viewport height in pixels.
    pub height: f64,
    /// Loaded image assets.
    pub assets: ImageAssets<I>,
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.chars().count() + 1 + word.chars().count() <= max_chars {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

fn draw_rounded_rect<C: Canvas>(
    ctx: &C,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    r: f64,
    bg: &str,
    border: &str,
    line_width: f64,
) {
    ctx.set_fill_style_str(bg);
    ctx.set_stroke_style_str(border);
    ctx.set_line_width(line_width);
    ctx.begin_path();
    ctx.move_to(x + r, y);
    ctx.line_to(x + w - r, y);
    let _ = ctx.arc_to(x + w, y, x + w, y + r, r);
    ctx.line_to(x + w, y + h - r);
    let _ = ctx.arc_to(x + w, y + h, x + w - r, y + h, r);
    ctx.line_to(x + r, y + h);
    let _ = ctx.arc_to(x, y + h, x, y + h - r, r);
    ctx.line_to(x, y + r);
    let _ = ctx.arc_to(x, y, x + r, y, r);
    ctx.close_path();
    ctx.fill();
    ctx.stroke();
}

fn draw_tail_down<C: Canvas>(
    ctx: &C,
    tail_x: f64,
    bubble_bottom_y: f64,
    tip_y: f64,
    bg: &str,
    border: &str,
    line_width: f64,
) {
    ctx.set_fill_style_str(bg);
    ctx.set_stroke_style_str(border);
    ctx.set_line_width(line_width);
    ctx.begin_path();
    ctx.move_to(tail_x - 14.0, bubble_bottom_y - 1.0);
    ctx.line_to(tail_x + 14.0, bubble_bottom_y - 1.0);
    ctx.line_to(tail_x, tip_y);
    ctx.close_path();
    ctx.fill();
    ctx.stroke();
    ctx.fill_rect(tail_x - 12.0, bubble_bottom_y - 3.0, 24.0, 5.0);
}

fn draw_tail_left<C: Canvas>(
    ctx: &C,
    bubble_left_x: f64,
    tail_y: f64,
    tip_x: f64,
    bg: &str,
    border: &str,
    line_width: f64,
) {
    ctx.set_fill_style_str(bg);
    ctx.set_stroke_style_str(border);
    ctx.set_line_width(line_width);
    ctx.begin_path();
    ctx.move_to(bubble_left_x + 1.0, tail_y - 12.0);
    ctx.line_to(bubble_left_x + 1.0, tail_y + 12.0);
    ctx.line_to(tip_x, tail_y);
    ctx.close_path();
    ctx.fill();
    ctx.stroke();
    ctx.fill_rect(bubble_left_x - 2.0, tail_y - 10.0, 5.0, 20.0);
}

impl<I> Default for ImageAssets<I> {
    fn default() -> Self {
        Self {
            bg_layer1: None,
            bg_layer2: None,
            bg_layer3: None,
            obstacle_hurdle: None,
            obstacle_meeting: None,
            obstacle_cactus: None,
            player_idle: None,
            player_walk1: None,
            player_walk2: None,
            player_jump: None,
            player_victory: None,
            host_idle: None,
            host_walk1: None,
            host_walk2: None,
            host_jump: None,
            host_victory: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Renderer<HtmlImageElement> {
    /// Creates a new `Renderer` targeting the specified canvas dimensions and loading level image assets.
    pub fn new(width: f64, height: f64, level: Option<&Level>) -> Self {
        Self::with_assets(width, height, ImageAssets::load(level))
    }
}

impl<I> Renderer<I> {
    /// Creates a renderer with already prepared image assets.
    pub fn with_assets(width: f64, height: f64, assets: ImageAssets<I>) -> Self {
        Self {
            width,
            height,
            assets,
        }
    }

    /// Renders the complete game view including background, track, entities, dialogue, and HUD.
    pub fn render<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
        // Clear screen
        ctx.clear_rect(0.0, 0.0, self.width, self.height);

        // 1. Sky & Sun (Layer 1)
        self.draw_sky(ctx, game.distance);

        // 2. Parallax Background Layers (Layer 2 & 3)
        self.draw_distant_hills(ctx, game.distance * PARALLAX_HILLS_SPEED);
        self.draw_midground_trees(ctx, game.distance * PARALLAX_TREES_SPEED);

        // 3. Ground & Running Track
        self.draw_track(ctx, game.distance);

        // 4. Checkpoints & Gates
        self.draw_checkpoints(ctx, game);

        // 5. Pickups
        self.draw_pickups(ctx, game);

        // 6. Obstacles (separate image assets)
        self.draw_obstacles(ctx, game);

        // 7. Host Runner (sprite poses)
        self.draw_host(ctx, game);

        // 8. Player Runner (sprite poses)
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

    fn draw_sky<C: Canvas<Image = I>>(&self, ctx: &C, distance: f64) {
        if let Some(img) = is_image_ready(ctx, &self.assets.bg_layer1) {
            draw_tiled_layer(
                ctx,
                img,
                self.width,
                self.height,
                distance * PARALLAX_SKY_SPEED,
            );
            return;
        }

        // Fallback: Nordic morning sky
        ctx.set_fill_style_str("#a8e6cf");
        ctx.fill_rect(0.0, 0.0, self.width, GROUND_Y);

        ctx.set_fill_style_str("#dcedc1");
        ctx.fill_rect(
            0.0,
            GROUND_Y - SKY_LOWER_BAND_HEIGHT,
            self.width,
            SKY_LOWER_BAND_HEIGHT,
        );

        // Sun
        ctx.set_fill_style_str("#ffd32a");
        ctx.begin_path();
        let _ = ctx.arc(
            SUN_CENTER_X,
            SUN_CENTER_Y,
            SUN_RADIUS,
            0.0,
            std::f64::consts::PI * 2.0,
        );
        ctx.fill();
    }

    fn draw_distant_hills<C: Canvas<Image = I>>(&self, ctx: &C, offset: f64) {
        if let Some(img) = is_image_ready(ctx, &self.assets.bg_layer2) {
            draw_tiled_layer(ctx, img, self.width, self.height, offset);
            return;
        }

        // Fallback: procedural hills
        ctx.set_fill_style_str("#8bc5cc");
        ctx.begin_path();
        ctx.move_to(0.0, GROUND_Y);

        let step = HILLS_STEP;
        let mut x = -(offset % step) - step;
        while x < self.width + step * 2.0 {
            let hill_h =
                HILLS_BASE_HEIGHT + ((x + offset) * HILLS_FREQUENCY).sin() * HILLS_AMPLITUDE;
            ctx.quadratic_curve_to(x + step * 0.5, GROUND_Y - hill_h, x + step, GROUND_Y);
            x += step;
        }
        ctx.line_to(self.width, GROUND_Y);
        ctx.close_path();
        ctx.fill();
    }

    fn draw_midground_trees<C: Canvas<Image = I>>(&self, ctx: &C, offset: f64) {
        if let Some(img) = is_image_ready(ctx, &self.assets.bg_layer3) {
            draw_tiled_layer(ctx, img, self.width, self.height, offset);
            return;
        }

        // Fallback: procedural trees
        ctx.set_fill_style_str("#5a9e78");
        let tree_spacing = TREE_SPACING;
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

    fn draw_track<C: Canvas<Image = I>>(&self, ctx: &C, distance: f64) {
        // Ground asphalt
        ctx.set_fill_style_str("#2f3640");
        ctx.fill_rect(0.0, GROUND_Y, self.width, self.height - GROUND_Y);

        // Grass verge
        ctx.set_fill_style_str("#44bd32");
        ctx.fill_rect(0.0, GROUND_Y, self.width, GRASS_VERGE_HEIGHT);

        // Running track lane lines & distance markers
        ctx.set_fill_style_str("#f5f6fa");
        let marker_spacing = LANE_MARKER_SPACING;
        let mut marker_x = -(distance % marker_spacing);
        while marker_x < self.width + marker_spacing {
            ctx.fill_rect(
                marker_x,
                GROUND_Y + LANE_MARKER_Y_OFFSET,
                LANE_MARKER_WIDTH,
                LANE_MARKER_HEIGHT,
            );
            marker_x += marker_spacing;
        }

        // Distance numbers along the pavement
        ctx.set_fill_style_str("#718093");
        ctx.set_font("bold 12px sans-serif");
        let meter_spacing = DISTANCE_METER_SPACING;
        let start_meter = (distance / meter_spacing).floor() * meter_spacing;
        for i in 0..DISTANCE_METER_LABEL_COUNT {
            let m = start_meter + (i as f64) * meter_spacing;
            let screen_x = m - distance + crate::game::DEFAULT_PLAYER_START_X;
            if screen_x >= 0.0 && screen_x <= self.width {
                let _ = ctx.fill_text(
                    &format!("{:.0}m", m),
                    screen_x,
                    GROUND_Y + DISTANCE_METER_Y_OFFSET,
                );
            }
        }
    }

    fn draw_checkpoints<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
        for (_, cp) in &game.active_checkpoints {
            let screen_x = cp.distance - game.distance + game.player.x;
            if screen_x > -CHECKPOINT_CULL_PADDING
                && screen_x < self.width + CHECKPOINT_CULL_PADDING
            {
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

    fn draw_pickups<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
        for pickup in &game.active_pickups {
            let screen_x = pickup.distance - game.distance + game.player.x;
            if screen_x > -PICKUP_CULL_PADDING && screen_x < self.width + PICKUP_CULL_PADDING {
                let y = GROUND_Y - crate::game::PICKUP_Y_OFFSET_FROM_GROUND;
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

    fn draw_obstacles<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
        for obs in &game.active_obstacles {
            let screen_x = obs.distance - game.distance + game.player.x;
            if screen_x > -OBSTACLE_CULL_PADDING && screen_x < self.width + OBSTACLE_CULL_PADDING {
                let y = GROUND_Y - obs.height;

                let img_opt = match obs.kind.as_str() {
                    "hurdle" => is_image_ready(ctx, &self.assets.obstacle_hurdle),
                    "meeting" => is_image_ready(ctx, &self.assets.obstacle_meeting),
                    "cactus" => is_image_ready(ctx, &self.assets.obstacle_cactus),
                    _ => None,
                };

                if let Some(img) = img_opt {
                    let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(
                        img, screen_x, y, obs.width, obs.height,
                    );
                } else if obs.kind == "hurdle" {
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

    fn draw_player<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
        let p = &game.player;
        let blink =
            p.invulnerability_timer > 0.0 && ((p.invulnerability_timer * 10.0) as i32 % 2 == 0);
        if blink {
            return;
        }

        let sprite_img = match &game.state {
            GameState::Victory { .. } => is_image_ready(ctx, &self.assets.player_victory),
            _ if !p.is_grounded => is_image_ready(ctx, &self.assets.player_jump),
            _ if p.animation_frame == 1 => is_image_ready(ctx, &self.assets.player_walk2),
            _ if p.animation_frame == 0 && (game.input.right || game.input.left) => {
                is_image_ready(ctx, &self.assets.player_walk1)
            }
            _ => is_image_ready(ctx, &self.assets.player_idle),
        };

        if let Some(img) = sprite_img {
            let sprite_w = RUNNER_HEIGHT * (80.0 / 110.0);
            let sprite_x = p.x - (sprite_w - RUNNER_WIDTH) / 2.0;
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(
                img,
                sprite_x,
                p.y,
                sprite_w,
                RUNNER_HEIGHT,
            );
        } else {
            // Draw Player Runner (Guest) procedural fallback
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
        }

        // Label tag
        ctx.set_fill_style_str("#2d3436");
        ctx.set_font("bold 11px sans-serif");
        let _ = ctx.fill_text("Sinä (Vieras)", p.x - 10.0, p.y - 6.0);
    }

    fn draw_host<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
        let h = &game.host;
        let host_x = game.player.x + h.x_offset;
        let host_y = h.y;

        let sprite_img = match &game.state {
            GameState::Victory { .. } => is_image_ready(ctx, &self.assets.host_victory),
            _ if !h.is_grounded => is_image_ready(ctx, &self.assets.host_jump),
            _ if h.animation_frame == 1 => is_image_ready(ctx, &self.assets.host_walk2),
            _ if h.animation_frame == 0 && (game.input.right || game.input.left) => {
                is_image_ready(ctx, &self.assets.host_walk1)
            }
            _ => is_image_ready(ctx, &self.assets.host_idle),
        };

        if let Some(img) = sprite_img {
            let sprite_w = RUNNER_HEIGHT * (80.0 / 110.0);
            let sprite_x = host_x - (sprite_w - RUNNER_WIDTH) / 2.0;
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(
                img,
                sprite_x,
                host_y,
                sprite_w,
                RUNNER_HEIGHT,
            );
        } else {
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
        }

        // Label tag
        ctx.set_fill_style_str("#d63031");
        ctx.set_font("bold 11px sans-serif");
        let _ = ctx.fill_text("Jukka (Juokseva Johtaja)", host_x - 30.0, host_y - 6.0);
    }

    fn draw_dialogue<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
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

    fn draw_hud<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
        // HUD container top bar
        ctx.set_fill_style_str("rgba(47, 53, 66, 0.85)");
        ctx.fill_rect(0.0, 0.0, self.width, HUD_BAR_HEIGHT);

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
        ctx.fill_rect(
            STAMINA_BAR_BG_X,
            STAMINA_BAR_BG_Y,
            STAMINA_BAR_BG_WIDTH,
            STAMINA_BAR_BG_HEIGHT,
        );

        let stamina_pct = (game.player.stamina / game.player.max_stamina).clamp(0.0, 1.0);
        let bar_color = if stamina_pct > STAMINA_HEALTHY_RATIO {
            "#2ed573"
        } else if stamina_pct > STAMINA_WARNING_RATIO {
            "#ffa502"
        } else {
            "#ff4757"
        };
        ctx.set_fill_style_str(bar_color);
        ctx.fill_rect(
            STAMINA_BAR_FILL_X,
            STAMINA_BAR_FILL_Y,
            STAMINA_BAR_FILL_MAX_WIDTH * stamina_pct,
            STAMINA_BAR_FILL_HEIGHT,
        );

        // Score
        ctx.set_fill_style_str("#ffd32a");
        ctx.set_font("bold 15px sans-serif");
        let _ = ctx.fill_text(&format!("Pisteet: {}", game.player.score), 800.0, 28.0);
    }

    fn draw_start_screen<C: Canvas<Image = I>>(&self, ctx: &C, game: &Game) {
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
            "Liiku nuolinäppäimillä [→] / [←], hyppää esteiden yli [↑] tai [VÄLILYÖNTI].",
            center_x - 265.0,
            300.0,
        );

        ctx.set_fill_style_str("#2ed573");
        ctx.fill_rect(center_x - 170.0, 340.0, 340.0, 60.0);
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 22px sans-serif");
        let _ = ctx.fill_text("ALOTA LENKKI [→]", center_x - 110.0, 378.0);
    }

    fn draw_quiz_overlay<C: Canvas<Image = I>>(
        &self,
        ctx: &C,
        game: &Game,
        feedback: Option<&crate::game::QuizFeedback>,
    ) {
        // Unobtrusive in-game speech bubbles: runners, ground, and scenery remain completely visible!
        let cp = match game.current_checkpoint() {
            Some(c) => c,
            None => return,
        };

        let host_x = game.player.x + game.host.x_offset;

        // 1. Question speech bubble ABOVE the runners
        let q_x = QUIZ_QUESTION_BOX_X;
        let q_y = QUIZ_QUESTION_BOX_Y;
        let q_w = QUIZ_QUESTION_BOX_WIDTH;
        let q_h = QUIZ_QUESTION_BOX_HEIGHT;

        draw_rounded_rect(ctx, q_x, q_y, q_w, q_h, 12.0, "#ffffff", "#ea580c", 3.0);
        // Tail points down to Host runner
        let tail_target_x = (host_x + 20.0).clamp(q_x + 30.0, q_x + q_w - 30.0);
        draw_tail_down(
            ctx,
            tail_target_x,
            q_y + q_h,
            q_y + q_h + 30.0,
            "#ffffff",
            "#ea580c",
            3.0,
        );

        // Header badge
        ctx.set_fill_style_str("#ea580c");
        ctx.set_font("bold 14px sans-serif");
        let _ = ctx.fill_text("🎙️ JUKKA (JUOKSEVA JOHTAJA) KYSYY:", q_x + 18.0, q_y + 24.0);

        // Wrapped question text
        ctx.set_fill_style_str("#1e293b");
        ctx.set_font("bold 16px sans-serif");
        let q_lines = wrap_text(&cp.question, QUIZ_QUESTION_MAX_CHARS);
        for (idx, line) in q_lines.iter().enumerate().take(3) {
            let _ = ctx.fill_text(line, q_x + 18.0, q_y + 50.0 + (idx as f64 * 22.0));
        }

        // Timestamp hint if present
        if let Some(ts) = &cp.podcast_timestamp {
            ctx.set_fill_style_str("#64748b");
            ctx.set_font("italic 12px sans-serif");
            let _ = ctx.fill_text(
                &format!("(Kuuntele jaksosta kohdasta ~{})", ts),
                q_x + 18.0,
                q_y + q_h - 12.0,
            );
        }

        // 2. Answer choices NEXT TO the runners, stacked on top of each other
        let choice_x = QUIZ_CHOICE_BOX_X;
        let choice_w = QUIZ_CHOICE_BOX_WIDTH;
        let choice_h = QUIZ_CHOICE_BOX_HEIGHT;

        let opt_a = cp.options.first();
        let opt_b = cp.options.get(1);

        // Choice A / 1
        let a_y = QUIZ_CHOICE_A_Y;
        let (a_bg, a_border) = if let Some(_fb) = feedback {
            if opt_a.map(|o| o.is_correct).unwrap_or(false) {
                ("#ecfdf5", "#10b981")
            } else {
                ("#fef2f2", "#ef4444")
            }
        } else {
            ("#ffffff", "#0984e3")
        };
        draw_rounded_rect(
            ctx, choice_x, a_y, choice_w, choice_h, 10.0, a_bg, a_border, 2.5,
        );
        draw_tail_left(
            ctx,
            choice_x,
            a_y + 33.0,
            choice_x - 18.0,
            a_bg,
            a_border,
            2.5,
        );

        // Choice A pill badge "[1]"
        draw_rounded_rect(
            ctx,
            choice_x + 14.0,
            a_y + 16.0,
            38.0,
            34.0,
            6.0,
            "#0984e3",
            "#0984e3",
            1.0,
        );
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 16px sans-serif");
        let _ = ctx.fill_text("[1]", choice_x + 23.0, a_y + 39.0);

        // Choice A text
        if let Some(oa) = opt_a {
            ctx.set_fill_style_str("#0f172a");
            ctx.set_font("bold 15px sans-serif");
            let a_lines = wrap_text(&oa.text, QUIZ_CHOICE_MAX_CHARS);
            for (idx, line) in a_lines.iter().enumerate().take(2) {
                let _ = ctx.fill_text(line, choice_x + 62.0, a_y + 30.0 + (idx as f64 * 20.0));
            }
        }

        // Choice B / 2
        let b_y = QUIZ_CHOICE_B_Y;
        let (b_bg, b_border) = if let Some(_fb) = feedback {
            if opt_b.map(|o| o.is_correct).unwrap_or(false) {
                ("#ecfdf5", "#10b981")
            } else {
                ("#fef2f2", "#ef4444")
            }
        } else {
            ("#ffffff", "#0984e3")
        };
        draw_rounded_rect(
            ctx, choice_x, b_y, choice_w, choice_h, 10.0, b_bg, b_border, 2.5,
        );
        draw_tail_left(
            ctx,
            choice_x,
            b_y + 33.0,
            choice_x - 18.0,
            b_bg,
            b_border,
            2.5,
        );

        // Choice B pill badge "[2]"
        draw_rounded_rect(
            ctx,
            choice_x + 14.0,
            b_y + 16.0,
            38.0,
            34.0,
            6.0,
            "#0984e3",
            "#0984e3",
            1.0,
        );
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 16px sans-serif");
        let _ = ctx.fill_text("[2]", choice_x + 23.0, b_y + 39.0);

        // Choice B text
        if let Some(ob) = opt_b {
            ctx.set_fill_style_str("#0f172a");
            ctx.set_font("bold 15px sans-serif");
            let b_lines = wrap_text(&ob.text, QUIZ_CHOICE_MAX_CHARS);
            for (idx, line) in b_lines.iter().enumerate().take(2) {
                let _ = ctx.fill_text(line, choice_x + 62.0, b_y + 30.0 + (idx as f64 * 20.0));
            }
        }

        // Feedback banner floating below choices
        if let Some(fb) = feedback {
            let fb_color = if fb.is_correct { "#10b981" } else { "#ef4444" };
            draw_rounded_rect(
                ctx,
                choice_x,
                QUIZ_FEEDBACK_BOX_Y,
                choice_w,
                QUIZ_FEEDBACK_BOX_HEIGHT,
                8.0,
                fb_color,
                "#ffffff",
                2.0,
            );
            ctx.set_fill_style_str("#ffffff");
            ctx.set_font("bold 14px sans-serif");
            let prefix = if fb.is_correct {
                "✓ OIKEIN! (+150 pistettä): "
            } else {
                "✗ VÄÄRIN! (-20 kuntoa): "
            };
            let _ = ctx.fill_text(
                &format!("{}{}", prefix, fb.explanation),
                choice_x + 16.0,
                410.0,
            );
        }
    }

    fn draw_victory_screen<C: Canvas<Image = I>>(
        &self,
        ctx: &C,
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

    fn draw_game_over_screen<C: Canvas<Image = I>>(&self, ctx: &C, reason: &str) {
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
