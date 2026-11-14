//! Renderer tests against a recording canvas: every game state and sprite branch draws without the browser.

use std::cell::RefCell;

use juokseva_johtaja_game::canvas::{Canvas, CanvasError};
use juokseva_johtaja_game::game::{Game, GameState, QuizFeedback};
use juokseva_johtaja_game::level::{HostEventData, Level, ObstacleData, PickupData};
use juokseva_johtaja_game::render::{ImageAssets, Renderer};

const WIDTH: f64 = 960.0;
const HEIGHT: f64 = 540.0;

/// Test image: `None` size means still loading.
#[derive(Clone, Copy)]
struct FakeImage(Option<(f64, f64)>);

#[derive(Default)]
struct RecordingCanvas {
    calls: RefCell<Vec<String>>,
    texts: RefCell<Vec<String>>,
}

impl RecordingCanvas {
    fn record(&self, call: &str) {
        self.calls.borrow_mut().push(call.to_string());
    }
    fn count(&self, call: &str) -> usize {
        self.calls
            .borrow()
            .iter()
            .filter(|c| c.as_str() == call)
            .count()
    }
    fn has_text(&self, needle: &str) -> bool {
        self.texts.borrow().iter().any(|t| t.contains(needle))
    }
}

impl Canvas for RecordingCanvas {
    type Image = FakeImage;

    fn image_size(&self, image: &FakeImage) -> Option<(f64, f64)> {
        image.0
    }
    fn clear_rect(&self, _: f64, _: f64, _: f64, _: f64) {
        self.record("clear_rect");
    }
    fn fill_rect(&self, _: f64, _: f64, _: f64, _: f64) {
        self.record("fill_rect");
    }
    fn stroke_rect(&self, _: f64, _: f64, _: f64, _: f64) {
        self.record("stroke_rect");
    }
    fn set_fill_style_str(&self, _: &str) {
        self.record("fill_style");
    }
    fn set_stroke_style_str(&self, _: &str) {
        self.record("stroke_style");
    }
    fn set_font(&self, _: &str) {
        self.record("font");
    }
    fn set_line_width(&self, _: f64) {
        self.record("line_width");
    }
    fn fill_text(&self, text: &str, _: f64, _: f64) -> Result<(), CanvasError> {
        self.record("fill_text");
        self.texts.borrow_mut().push(text.to_string());
        Ok(())
    }
    fn begin_path(&self) {
        self.record("begin_path");
    }
    fn close_path(&self) {
        self.record("close_path");
    }
    fn fill(&self) {
        self.record("fill");
    }
    fn stroke(&self) {
        self.record("stroke");
    }
    fn move_to(&self, _: f64, _: f64) {
        self.record("move_to");
    }
    fn line_to(&self, _: f64, _: f64) {
        self.record("line_to");
    }
    fn quadratic_curve_to(&self, _: f64, _: f64, _: f64, _: f64) {
        self.record("quadratic_curve_to");
    }
    fn arc(&self, _: f64, _: f64, _: f64, _: f64, _: f64) -> Result<(), CanvasError> {
        self.record("arc");
        Ok(())
    }
    fn arc_to(&self, _: f64, _: f64, _: f64, _: f64, _: f64) -> Result<(), CanvasError> {
        self.record("arc_to");
        Ok(())
    }
    fn draw_image_with_html_image_element_and_dw_and_dh(
        &self,
        _: &FakeImage,
        _: f64,
        _: f64,
        _: f64,
        _: f64,
    ) -> Result<(), CanvasError> {
        self.record("draw_image");
        Ok(())
    }
}

fn loaded_assets() -> ImageAssets<FakeImage> {
    let image = || Some(FakeImage(Some((200.0, 100.0))));
    ImageAssets {
        bg_layer1: image(),
        bg_layer2: image(),
        bg_layer3: image(),
        obstacle_hurdle: image(),
        obstacle_meeting: image(),
        obstacle_cactus: image(),
        player_idle: image(),
        player_walk1: image(),
        player_walk2: image(),
        player_jump: image(),
        player_victory: image(),
        host_idle: image(),
        host_walk1: image(),
        host_walk2: image(),
        host_jump: image(),
        host_victory: image(),
    }
}

fn loading_assets() -> ImageAssets<FakeImage> {
    ImageAssets {
        bg_layer1: Some(FakeImage(None)),
        ..ImageAssets::default()
    }
}

/// A level with every obstacle and pickup kind right in front of the runner.
fn busy_level() -> Level {
    let mut level = Level::default_episode_1();
    level.obstacles = ["hurdle", "meeting", "cactus", "unknown"]
        .iter()
        .enumerate()
        .map(|(i, kind)| ObstacleData {
            distance: 150.0 + 120.0 * i as f64,
            kind: kind.to_string(),
            width: 40.0,
            height: 50.0,
            image: None,
        })
        .collect();
    level.pickups = ["coffee", "podcast_mic", "other"]
        .iter()
        .enumerate()
        .map(|(i, kind)| PickupData {
            distance: 200.0 + 100.0 * i as f64,
            kind: kind.to_string(),
            value: 5.0,
        })
        .collect();
    level.host_events = vec![HostEventData {
        distance: 0.0,
        action: "help".into(),
        dialogue: "Jaksaa!".into(),
    }];
    level.checkpoints[0].distance = 300.0;
    level
}

fn draw(renderer: &Renderer<FakeImage>, game: &Game) -> RecordingCanvas {
    let canvas = RecordingCanvas::default();
    renderer.render(&canvas, game);
    assert_eq!(canvas.count("clear_rect"), 1);
    canvas
}

fn all_states(game: &Game) -> Vec<Game> {
    let mut states = Vec::new();
    for state in [
        GameState::Start,
        GameState::Running,
        GameState::Quiz {
            checkpoint_index: 0,
            feedback: None,
        },
        GameState::Quiz {
            checkpoint_index: 0,
            feedback: Some(QuizFeedback {
                is_correct: true,
                explanation: "Hyvä vastaus, kuten jaksossa kerrottiin.".into(),
                timer: 1.0,
            }),
        },
        GameState::Quiz {
            checkpoint_index: 0,
            feedback: Some(QuizFeedback {
                is_correct: false,
                explanation: "Ei aivan.".into(),
                timer: 1.0,
            }),
        },
        GameState::Victory {
            score: 1234,
            time_seconds: 75.5,
        },
        GameState::GameOver {
            reason: "Voimat loppuivat".into(),
        },
    ] {
        let mut copy = game.clone();
        copy.state = state;
        states.push(copy);
    }
    states
}

#[test]
fn renders_every_state_with_loaded_sprites() {
    let renderer = Renderer::with_assets(WIDTH, HEIGHT, loaded_assets());
    let mut game = Game::new(busy_level());
    game.host.active_dialogue =
        Some("Tämä on pitkä puhekupla, joka rivitetään useammalle riville ruudulla.".into());
    for state in all_states(&game) {
        let canvas = draw(&renderer, &state);
        assert!(canvas.count("draw_image") > 0);
        assert!(canvas.count("fill_text") > 0);
    }

    let start = draw(&renderer, &all_states(&game)[0]);
    assert!(start.has_text("JUOKSEVA JOHTAJA"));
    let victory = draw(&renderer, &all_states(&game)[5]);
    assert!(victory.has_text("1234"));
    let game_over = draw(&renderer, &all_states(&game)[6]);
    assert!(game_over.has_text("Voimat loppuivat"));
}

#[test]
fn renders_fallback_shapes_while_images_load() {
    let renderer = Renderer::with_assets(WIDTH, HEIGHT, loading_assets());
    let game = Game::new(busy_level());
    for state in all_states(&game) {
        let canvas = draw(&renderer, &state);
        assert_eq!(
            canvas.count("draw_image"),
            0,
            "nothing is drawn from images that are not ready"
        );
        assert!(canvas.count("fill_rect") > 0);
    }
}

#[test]
fn renders_sprite_poses() {
    for assets in [loaded_assets(), ImageAssets::default()] {
        let renderer = Renderer::with_assets(WIDTH, HEIGHT, assets);
        let mut game = Game::new(busy_level());
        game.start();

        // Running frames 0 and 1, moving forward.
        game.input.right = true;
        for frame in [0, 1] {
            game.player.animation_frame = frame;
            game.host.animation_frame = frame;
            draw(&renderer, &game);
        }

        // Airborne, invulnerable after a hit, host helping and sabotaging.
        game.player.is_grounded = false;
        game.host.is_grounded = false;
        game.player.invulnerability_timer = 0.5;
        for (helping, sabotaging) in [(true, false), (false, true)] {
            game.host.is_helping = helping;
            game.host.is_sabotaging = sabotaging;
            draw(&renderer, &game);
        }

        // Low stamina HUD and far along the track.
        game.player.stamina = 5.0;
        game.distance = game.level.track_length - 100.0;
        draw(&renderer, &game);
    }
}

#[test]
fn skips_zero_sized_background_tiles() {
    let mut assets = loaded_assets();
    assets.bg_layer1 = Some(FakeImage(Some((0.0, 0.0))));
    let renderer = Renderer::with_assets(WIDTH, HEIGHT, assets);
    draw(&renderer, &Game::new(busy_level()));
}
