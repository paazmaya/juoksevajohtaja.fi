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
use crate::render::Renderer;

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
    let game = Rc::new(RefCell::new(Game::new(default_level)));
    let renderer = Renderer::new(width, height);
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
        if let Some(modal) = doc.get_element_by_id("quiz-overlay") {
            match &g.state {
                GameState::Quiz {
                    checkpoint_index,
                    feedback,
                } => {
                    let _ = modal.class_list().remove_1("hidden");
                    if let Some(cp) = g.level.checkpoints.get(*checkpoint_index) {
                        if let Some(q_el) = doc.get_element_by_id("quiz-question-text") {
                            q_el.set_text_content(Some(&cp.question));
                        }
                        if let Some(btn_a) = doc.get_element_by_id("btn-choice-a") {
                            if let Some(opt_a) = cp.options.get(0) {
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
                    let _ = modal.class_list().add_1("hidden");
                }
            }
        }
    }

    // Keyboard controls
    {
        let game_clone = game.clone();
        let doc_clone = document.clone();
        let keydown_closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                let key = event.key();
                let mut g = game_clone.borrow_mut();
                match key.as_str() {
                    " " | "ArrowUp" | "w" | "W" => {
                        if g.state == GameState::Start {
                            g.start();
                        } else if g.state == GameState::Running {
                            g.jump();
                        }
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

    // Click / Touch on Canvas
    {
        let game_clone = game.clone();
        let doc_clone = document.clone();
        let click_closure =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_event: web_sys::MouseEvent| {
                let mut g = game_clone.borrow_mut();
                if g.state == GameState::Start {
                    g.start();
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
        let dt = ((now - last_time) / 1000.0).clamp(0.001, 0.1);
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
