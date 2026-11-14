//! State machine edge cases: input handling, quiz outcomes, backward running, stamina and progress.

use juokseva_johtaja_game::audio::SoundManager;
use juokseva_johtaja_game::game::{AudioCue, Game, GameState, QuizFeedback};
use juokseva_johtaja_game::level::{Choice, Level, ObstacleData, QuestionCheckpoint};

/// Episode 1 without checkpoints, obstacles, pickups or host events, for isolated physics tests.
fn empty_level() -> Level {
    let mut level = Level::default_episode_1();
    level.checkpoints.clear();
    level.obstacles.clear();
    level.pickups.clear();
    level.host_events.clear();
    level
}

fn running(level: Level) -> Game {
    let mut game = Game::new(level);
    game.start();
    game
}

#[test]
fn set_input_jumps_only_while_running() {
    let mut game = Game::new(empty_level());
    game.set_input(false, false, true);
    assert!(game.player.is_grounded, "no jump before the run starts");
    assert!(game.input.up);

    game.start();
    game.set_input(true, false, true);
    assert!(!game.player.is_grounded);
    assert_eq!(game.last_audio_cue, AudioCue::Jump);
    assert!(game.input.right);
    assert!(!game.input.left);
}

#[test]
fn current_checkpoint_only_during_quiz() {
    let mut game = Game::new(Level::default_episode_1());
    assert!(game.current_checkpoint().is_none());
    game.state = GameState::Quiz {
        checkpoint_index: 0,
        feedback: None,
    };
    assert_eq!(
        game.current_checkpoint().map(|c| c.id.clone()),
        Some(game.level.checkpoints[0].id.clone())
    );
}

#[test]
fn quiz_uses_default_explanations() {
    let mut level = Level::default_episode_1();
    for checkpoint in &mut level.checkpoints {
        checkpoint.explanation = None;
    }
    let correct_id = level.checkpoints[0]
        .options
        .iter()
        .find(|o| o.is_correct)
        .unwrap()
        .id
        .clone();
    let wrong_id = level.checkpoints[0]
        .options
        .iter()
        .find(|o| !o.is_correct)
        .unwrap()
        .id
        .clone();

    let mut game = Game::new(level.clone());
    game.state = GameState::Quiz {
        checkpoint_index: 0,
        feedback: None,
    };
    assert!(game.answer_quiz(&correct_id));
    match &game.state {
        GameState::Quiz {
            feedback: Some(QuizFeedback { explanation, .. }),
            ..
        } => assert!(explanation.starts_with("Oikein")),
        other => panic!("unexpected state {other:?}"),
    }

    let mut game = Game::new(level);
    game.state = GameState::Quiz {
        checkpoint_index: 0,
        feedback: None,
    };
    assert!(!game.answer_quiz(&wrong_id));
    match &game.state {
        GameState::Quiz {
            feedback: Some(QuizFeedback { explanation, .. }),
            ..
        } => assert!(explanation.starts_with("Väärin")),
        other => panic!("unexpected state {other:?}"),
    }
}

#[test]
fn answers_are_ignored_outside_a_valid_quiz() {
    let mut game = Game::new(Level::default_episode_1());
    assert!(!game.answer_quiz("A"), "not in a quiz");
    game.state = GameState::Quiz {
        checkpoint_index: 999,
        feedback: None,
    };
    assert!(!game.answer_quiz("A"), "checkpoint does not exist");
}

#[test]
fn wrong_answer_without_stamina_ends_the_game() {
    let mut game = Game::new(Level::default_episode_1());
    game.player.stamina = 0.0;
    game.state = GameState::Quiz {
        checkpoint_index: 0,
        feedback: Some(QuizFeedback {
            is_correct: false,
            explanation: String::new(),
            timer: 0.1,
        }),
    };
    game.update(0.2);
    assert!(matches!(game.state, GameState::GameOver { .. }));
    assert_eq!(game.last_audio_cue, AudioCue::GameOver);
}

#[test]
fn finished_and_waiting_states_do_not_advance() {
    let mut game = Game::new(empty_level());
    game.input.right = true;
    game.update(1.0);
    assert_eq!(game.distance, 0.0, "start screen waits for input");

    for state in [
        GameState::Victory {
            score: 1,
            time_seconds: 2.0,
        },
        GameState::GameOver { reason: "x".into() },
    ] {
        game.state = state;
        game.update(1.0);
        assert_eq!(game.distance, 0.0);
    }
}

#[test]
fn running_forward_and_backward_animates_and_drains_stamina() {
    let mut game = running(empty_level());
    game.input.right = true;
    game.update(0.25);
    assert_eq!(game.player.animation_frame, 1);
    assert_eq!(game.host.animation_frame, 1);
    let forward_distance = game.distance;

    game.input.right = false;
    game.input.left = true;
    let stamina_before = game.player.stamina;
    game.update(0.25);
    assert!(game.distance < forward_distance);
    assert!(game.player.stamina < stamina_before);
    assert_eq!(game.player.animation_frame, 0);
    assert_eq!(game.host.animation_frame, 0);

    game.update(10.0);
    assert_eq!(game.distance, 0.0, "cannot run behind the start line");
}

#[test]
fn running_out_of_stamina_ends_the_game() {
    let mut game = running(empty_level());
    game.player.stamina = 0.01;
    game.input.right = true;
    game.update(1.0);
    assert!(matches!(game.state, GameState::GameOver { .. }));
    assert_eq!(game.last_audio_cue, AudioCue::GameOver);
}

#[test]
fn progress_is_a_clamped_percentage() {
    let mut game = Game::new(empty_level());
    assert_eq!(game.progress_percent(), 0.0);
    game.distance = game.level.track_length / 2.0;
    assert_eq!(game.progress_percent(), 50.0);
    game.distance = game.level.track_length * 3.0;
    assert_eq!(game.progress_percent(), 100.0);
    game.level.track_length = 0.0;
    assert_eq!(game.progress_percent(), 0.0);
}

#[test]
fn validation_rejects_broken_levels() {
    let mut level = empty_level();
    level.track_length = 0.0;
    assert_eq!(
        level.validate().unwrap_err(),
        "track_length must be positive"
    );

    let mut level = empty_level();
    level.checkpoints.push(QuestionCheckpoint {
        id: "solo".into(),
        distance: 10.0,
        question: "?".into(),
        options: vec![Choice {
            id: "A".into(),
            text: "a".into(),
            is_correct: true,
        }],
        podcast_timestamp: None,
        explanation: None,
    });
    assert!(level.validate().unwrap_err().contains("fewer than"));

    let mut level = empty_level();
    level.obstacles.push(ObstacleData {
        distance: level.track_length + 1.0,
        kind: "hurdle".into(),
        width: 10.0,
        height: 10.0,
        image: None,
    });
    assert!(level
        .validate()
        .unwrap_err()
        .contains("out of track bounds"));
}

#[test]
fn sound_effects_are_safe_to_call_without_web_audio() {
    let mut sounds = SoundManager::default();
    assert!(!sounds.muted);
    sounds.muted = true;
    sounds.play_jump();
    sounds.play_pickup();
    sounds.play_quiz_correct();
    sounds.play_quiz_wrong();
    sounds.play_obstacle_hit();
    sounds.play_victory();
    sounds.play_game_over();
    assert!(SoundManager::new().muted == false);
}
