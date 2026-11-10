//! WebAssembly browser entry point, DOM event listeners, and main animation loop for Juokseva Johtaja.

pub mod audio;
pub mod game;
pub mod level;
pub mod physics;
pub mod render;

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use crate::audio::SoundManager;
#[cfg(target_arch = "wasm32")]
use crate::game::{AudioCue, Game, GameState};
#[cfg(target_arch = "wasm32")]
use crate::level::Level;
#[cfg(target_arch = "wasm32")]
use crate::render::{
    Renderer, QUIZ_CHOICE_A_Y, QUIZ_CHOICE_BOX_HEIGHT, QUIZ_CHOICE_BOX_WIDTH, QUIZ_CHOICE_BOX_X,
    QUIZ_CHOICE_B_Y, QUIZ_CLICK_TOLERANCE,
};

/// Milliseconds in one second for performance timestamp conversions.
pub const MILLISECONDS_PER_SECOND: f64 = 1000.0;

/// Minimum delta time per animation frame in seconds (avoids zero or negative dt).
pub const MIN_FRAME_DELTA_SECONDS: f64 = 0.001;

/// Maximum delta time clamped per animation frame in seconds (prevents physics explosion after tab switch).
pub const MAX_FRAME_DELTA_SECONDS: f64 = 0.1;

/// WebAssembly entry point initializing canvas context, game loops, and keyboard/touch event handlers.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start_game() -> Result<(), JsValue> {
    let window = web_sys::window().expect("global window does not exist");
    let document = window.document().expect("document does not exist");

    let canvas = document
        .get_element_by_id("game-canvas")
        .expect("element #game-canvas not found")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .expect("2d context not found")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    let default_level = Level::default_episode_1();
    let renderer = Renderer::new(width, height, Some(&default_level));
    let game = Rc::new(RefCell::new(Game::new(default_level)));
    let sound = Rc::new(RefCell::new(SoundManager::new()));

    // Synchronize DOM UI overlay elements if present (for Playwright testing and accessibility)
    fn update_dom_status(doc: &web_sys::Document, g: &Game) {
        if let Some(el) = doc.get_element_by_id("game-state-label") {
            let label = match &g.state {
                GameState::Start => "START",
                GameState::Running => "RUNNING",
                GameState::Quiz { .. } => "QUIZ",
                GameState::Victory { .. } => "VICTORY",
                GameState::GameOver { .. } => "GAMEOVER",
            };
            el.set_text_content(Some(label));
        }
        if let Some(el) = doc.get_element_by_id("score-value") {
            el.set_text_content(Some(&g.player.score.to_string()));
        }
        if let Some(el) = doc.get_element_by_id("distance-value") {
            el.set_text_content(Some(&format!("{:.0}", g.distance)));
        }
        if let Some(layer) = doc.get_element_by_id("quiz-interactive-layer") {
            match &g.state {
                GameState::Quiz {
                    checkpoint_index,
                    feedback,
                } => {
                    let _ = layer.class_list().remove_1("hidden");
                    if let Some(cp) = g.level.checkpoints.get(*checkpoint_index) {
                        if let Some(q_el) = doc.get_element_by_id("quiz-question-text") {
                            q_el.set_text_content(Some(&cp.question));
                        }
                        if let Some(btn_a) = doc.get_element_by_id("btn-choice-a") {
                            if let Some(opt_a) = cp.options.first() {
                                btn_a.set_text_content(Some(&format!("[1] {}", opt_a.text)));
                            }
                        }
                        if let Some(btn_b) = doc.get_element_by_id("btn-choice-b") {
                            if let Some(opt_b) = cp.options.get(1) {
                                btn_b.set_text_content(Some(&format!("[2] {}", opt_b.text)));
                            }
                        }
                    }
                    if let Some(fb_el) = doc.get_element_by_id("quiz-feedback-box") {
                        if let Some(fb) = feedback {
                            let _ = fb_el.class_list().remove_1("hidden");
                            let msg = if fb.is_correct {
                                format!("Oikein! {}", fb.explanation)
                            } else {
                                format!("Väärin! {}", fb.explanation)
                            };
                            fb_el.set_text_content(Some(&msg));
                        } else {
                            let _ = fb_el.class_list().add_1("hidden");
                        }
                    }
                }
                _ => {
                    let _ = layer.class_list().add_1("hidden");
                }
            }
        }
    }

    // Keyboard controls: keydown
    {
        let game_clone = game.clone();
        let doc_clone = document.clone();
        let keydown_closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                let key = event.key();
                let mut g = game_clone.borrow_mut();
                match key.as_str() {
                    "ArrowRight" | "d" | "D" => {
                        if g.state == GameState::Start {
                            g.start();
                        }
                        g.input.right = true;
                        event.prevent_default();
                    }
                    "ArrowLeft" | "a" | "A" => {
                        g.input.left = true;
                        event.prevent_default();
                    }
                    " " | "ArrowUp" | "w" | "W" => {
                        if g.state == GameState::Start {
                            g.start();
                        } else if g.state == GameState::Running {
                            g.jump();
                        }
                        g.input.up = true;
                        event.prevent_default();
                    }
                    "1" => {
                        if let GameState::Quiz { .. } = &g.state {
                            g.answer_quiz("A");
                        }
                    }
                    "2" => {
                        if let GameState::Quiz { .. } = &g.state {
                            g.answer_quiz("B");
                        }
                    }
                    "r" | "R" => {
                        if matches!(
                            g.state,
                            GameState::GameOver { .. } | GameState::Victory { .. }
                        ) {
                            let level = g.level.clone();
                            *g = Game::new(level);
                            g.start();
                        }
                    }
                    "Escape" => {
                        if let Some(modal) = doc_clone.get_element_by_id("about-overlay") {
                            let _ = modal.class_list().add_1("hidden");
                        }
                    }
                    _ => {}
                }
                update_dom_status(&doc_clone, &g);
            },
        );
        window.add_event_listener_with_callback(
            "keydown",
            keydown_closure.as_ref().unchecked_ref(),
        )?;
        keydown_closure.forget();
    }

    // Keyboard controls: keyup
    {
        let game_clone = game.clone();
        let keyup_closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                let key = event.key();
                let mut g = game_clone.borrow_mut();
                match key.as_str() {
                    "ArrowRight" | "d" | "D" => {
                        g.input.right = false;
                    }
                    "ArrowLeft" | "a" | "A" => {
                        g.input.left = false;
                    }
                    " " | "ArrowUp" | "w" | "W" => {
                        g.input.up = false;
                    }
                    _ => {}
                }
            },
        );
        window.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())?;
        keyup_closure.forget();
    }

    // Window blur: reset input to prevent stuck keys
    {
        let game_clone = game.clone();
        let blur_closure = Closure::<dyn FnMut()>::new(move || {
            let mut g = game_clone.borrow_mut();
            g.input.right = false;
            g.input.left = false;
            g.input.up = false;
        });
        window.add_event_listener_with_callback("blur", blur_closure.as_ref().unchecked_ref())?;
        blur_closure.forget();
    }

    // Canvas click
    {
        let game_clone = game.clone();
        let doc_clone = document.clone();
        let canvas_ref = canvas.clone();
        let click_closure =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                let mut g = game_clone.borrow_mut();
                let rect = canvas_ref.get_bounding_client_rect();
                let scale_x = canvas_ref.width() as f64 / rect.width();
                let scale_y = canvas_ref.height() as f64 / rect.height();
                let click_x = (event.client_x() as f64 - rect.left()) * scale_x;
                let click_y = (event.client_y() as f64 - rect.top()) * scale_y;

                if g.state == GameState::Start {
                    g.start();
                } else if matches!(g.state, GameState::Quiz { .. }) {
                    // Check if clicked choice A or B on canvas
                    let choice_left = QUIZ_CHOICE_BOX_X - QUIZ_CLICK_TOLERANCE * 2.0;
                    let choice_right = QUIZ_CHOICE_BOX_X + QUIZ_CHOICE_BOX_WIDTH;
                    let a_top = QUIZ_CHOICE_A_Y - QUIZ_CLICK_TOLERANCE;
                    let a_bottom = QUIZ_CHOICE_A_Y + QUIZ_CHOICE_BOX_HEIGHT + QUIZ_CLICK_TOLERANCE;
                    let b_top = QUIZ_CHOICE_B_Y - QUIZ_CLICK_TOLERANCE;
                    let b_bottom = QUIZ_CHOICE_B_Y + QUIZ_CHOICE_BOX_HEIGHT + QUIZ_CLICK_TOLERANCE;

                    if click_x >= choice_left && click_x <= choice_right {
                        if click_y >= a_top && click_y <= a_bottom {
                            g.answer_quiz("A");
                        } else if click_y >= b_top && click_y <= b_bottom {
                            g.answer_quiz("B");
                        }
                    }
                } else if g.state == GameState::Running {
                    g.jump();
                } else if matches!(
                    g.state,
                    GameState::GameOver { .. } | GameState::Victory { .. }
                ) {
                    let level = g.level.clone();
                    *g = Game::new(level);
                    g.start();
                }
                update_dom_status(&doc_clone, &g);
            });
        canvas.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
        click_closure.forget();
    }

    // Bind DOM quiz buttons
    {
        let game_clone = game.clone();
        let doc_clone = document.clone();
        if let Some(btn_a) = document.get_element_by_id("btn-choice-a") {
            let g = game_clone.clone();
            let d = doc_clone.clone();
            let cb = Closure::<dyn FnMut()>::new(move || {
                let mut gm = g.borrow_mut();
                gm.answer_quiz("A");
                update_dom_status(&d, &gm);
            });
            btn_a.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }

        if let Some(btn_b) = document.get_element_by_id("btn-choice-b") {
            let g = game_clone.clone();
            let d = doc_clone.clone();
            let cb = Closure::<dyn FnMut()>::new(move || {
                let mut gm = g.borrow_mut();
                gm.answer_quiz("B");
                update_dom_status(&d, &gm);
            });
            btn_b.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }

        if let Some(btn_jump) = document.get_element_by_id("btn-jump-touch") {
            let g = game_clone.clone();
            let d = doc_clone.clone();
            let cb = Closure::<dyn FnMut()>::new(move || {
                let mut gm = g.borrow_mut();
                if gm.state == GameState::Start {
                    gm.start();
                } else if gm.state == GameState::Running {
                    gm.jump();
                }
                update_dom_status(&d, &gm);
            });
            btn_jump.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }

        // On-screen touch left button
        if let Some(btn_left) = document.get_element_by_id("btn-touch-left") {
            let g = game_clone.clone();
            let cb_down = Closure::<dyn FnMut()>::new({
                let g = g.clone();
                move || {
                    g.borrow_mut().input.left = true;
                }
            });
            let cb_up = Closure::<dyn FnMut()>::new({
                let g = g.clone();
                move || {
                    g.borrow_mut().input.left = false;
                }
            });
            btn_left.add_event_listener_with_callback(
                "pointerdown",
                cb_down.as_ref().unchecked_ref(),
            )?;
            btn_left
                .add_event_listener_with_callback("pointerup", cb_up.as_ref().unchecked_ref())?;
            btn_left.add_event_listener_with_callback(
                "pointercancel",
                cb_up.as_ref().unchecked_ref(),
            )?;
            btn_left
                .add_event_listener_with_callback("pointerleave", cb_up.as_ref().unchecked_ref())?;
            cb_down.forget();
            cb_up.forget();
        }

        // On-screen touch right button
        if let Some(btn_right) = document.get_element_by_id("btn-touch-right") {
            let g = game_clone.clone();
            let cb_down = Closure::<dyn FnMut()>::new({
                let g = g.clone();
                move || {
                    let mut gm = g.borrow_mut();
                    if gm.state == GameState::Start {
                        gm.start();
                    }
                    gm.input.right = true;
                }
            });
            let cb_up = Closure::<dyn FnMut()>::new({
                let g = g.clone();
                move || {
                    g.borrow_mut().input.right = false;
                }
            });
            btn_right.add_event_listener_with_callback(
                "pointerdown",
                cb_down.as_ref().unchecked_ref(),
            )?;
            btn_right
                .add_event_listener_with_callback("pointerup", cb_up.as_ref().unchecked_ref())?;
            btn_right.add_event_listener_with_callback(
                "pointercancel",
                cb_up.as_ref().unchecked_ref(),
            )?;
            btn_right
                .add_event_listener_with_callback("pointerleave", cb_up.as_ref().unchecked_ref())?;
            cb_down.forget();
            cb_up.forget();
        }

        // About / Instructions modal toggle
        if let Some(btn_inst) = document.get_element_by_id("btn-instructions") {
            let d = doc_clone.clone();
            let cb = Closure::<dyn FnMut()>::new(move || {
                if let Some(modal) = d.get_element_by_id("about-overlay") {
                    let _ = modal.class_list().remove_1("hidden");
                }
            });
            btn_inst.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }

        if let Some(btn_close) = document.get_element_by_id("btn-close-about") {
            let d = doc_clone.clone();
            let cb = Closure::<dyn FnMut()>::new(move || {
                if let Some(modal) = d.get_element_by_id("about-overlay") {
                    let _ = modal.class_list().add_1("hidden");
                }
            });
            btn_close.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }

        if let Some(btn_cont) = document.get_element_by_id("btn-about-continue") {
            let d = doc_clone.clone();
            let cb = Closure::<dyn FnMut()>::new(move || {
                if let Some(modal) = d.get_element_by_id("about-overlay") {
                    let _ = modal.class_list().add_1("hidden");
                }
            });
            btn_cont.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }

        if let Some(about_modal) = document.get_element_by_id("about-overlay") {
            let modal_ref = about_modal.clone();
            let cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(
                move |event: web_sys::MouseEvent| {
                    if let Some(target) = event.target() {
                        if let Ok(target_el) = target.dyn_into::<web_sys::Element>() {
                            if target_el.id() == "about-overlay" {
                                let _ = modal_ref.class_list().add_1("hidden");
                            }
                        }
                    }
                },
            );
            about_modal.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }
    }

    // Main animation loop
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let game_loop = game.clone();
    let sound_loop = sound.clone();
    let doc_loop = document.clone();
    let win_loop = window.clone();

    let mut last_time = window.performance().map(|p| p.now()).unwrap_or(0.0);

    *g.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
        let now = win_loop.performance().map(|p| p.now()).unwrap_or(0.0);
        let dt = ((now - last_time) / MILLISECONDS_PER_SECOND)
            .clamp(MIN_FRAME_DELTA_SECONDS, MAX_FRAME_DELTA_SECONDS);
        last_time = now;

        {
            let mut gm = game_loop.borrow_mut();
            gm.update(dt);

            // Handle audio cue triggers
            let snd = sound_loop.borrow();
            match gm.last_audio_cue {
                AudioCue::Jump => snd.play_jump(),
                AudioCue::Pickup => snd.play_pickup(),
                AudioCue::QuizCorrect => snd.play_quiz_correct(),
                AudioCue::QuizWrong => snd.play_quiz_wrong(),
                AudioCue::ObstacleHit => snd.play_obstacle_hit(),
                AudioCue::Victory => snd.play_victory(),
                AudioCue::GameOver => snd.play_game_over(),
                AudioCue::None => {}
            }

            renderer.render(&ctx, &gm);
            update_dom_status(&doc_loop, &gm);
        }

        // Request next frame
        if let Some(ref callback) = *f.borrow() {
            let _ = win_loop.request_animation_frame(callback.as_ref().unchecked_ref());
        }
    }));

    if let Some(ref callback) = *g.borrow() {
        let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
    }

    Ok(())
}
