use juokseva_johtaja_game::game::{AudioCue, Game, GameState};
use juokseva_johtaja_game::level::Level;

#[test]
fn test_game_initialization_and_start() {
    let level = Level::default_episode_1();
    let mut game = Game::new(level);
    assert_eq!(game.state, GameState::Start);
    assert_eq!(game.distance, 0.0);
    assert_eq!(game.player.stamina, 100.0);

    game.start();
    assert_eq!(game.state, GameState::Running);
}

#[test]
fn test_player_jump_kinematics() {
    let level = Level::default_episode_1();
    let mut game = Game::new(level);
    game.start();

    assert!(game.player.is_grounded);
    game.jump();
    assert_eq!(game.last_audio_cue, AudioCue::Jump);
    assert!(!game.player.is_grounded);
    assert!(game.player.vy < 0.0); // Moving upwards

    // Simulate 0.2s of jump in air
    game.update(0.2);
    assert!(!game.player.is_grounded);

    // Simulate another 1.0s, runner should land back on ground
    for _ in 0..10 {
        game.update(0.1);
    }
    assert!(game.player.is_grounded);
    assert_eq!(game.player.vy, 0.0);
}

#[test]
fn test_pickup_collection() {
    let level = Level::default_episode_1();
    let mut game = Game::new(level);
    game.start();

    let initial_pickup_count = game.active_pickups.len();
    assert!(initial_pickup_count > 0);

    // Drain some stamina to test replenishment
    game.player.stamina = 50.0;

    // Simulate running past the first pickup (located at ~150m)
    for _ in 0..25 {
        game.update(0.05); // ~1.25s of running at ~260px/s => ~325px
    }

    // Pickup should have been consumed
    assert!(game.active_pickups.len() < initial_pickup_count);
    assert!(game.player.stamina > 50.0);
    assert!(game.player.score > 0);
}

#[test]
fn test_quiz_checkpoint_correct_answer_advances_game() {
    let level = Level::default_episode_1();
    let mut game = Game::new(level);
    game.start();

    // First checkpoint is at distance 450
    // Advance game until quiz triggers
    while game.distance < 450.0 && game.state == GameState::Running {
        game.update(0.05);
    }

    assert!(matches!(game.state, GameState::Quiz { .. }));
    let initial_score = game.player.score;

    // Answer correctly: Choice "A" in episode 1 Q1 is correct
    let is_correct = game.answer_quiz("A");
    assert!(is_correct);
    assert_eq!(game.last_audio_cue, AudioCue::QuizCorrect);
    assert_eq!(game.player.score, initial_score + 150);

    // Elapse feedback timer to return to running
    game.update(2.1);
    assert_eq!(game.state, GameState::Running);
    assert!(game.distance >= 450.0);
}

#[test]
fn test_quiz_checkpoint_wrong_answer_penalizes_and_retries() {
    let level = Level::default_episode_1();
    let mut game = Game::new(level);
    game.start();

    // Advance until checkpoint 1 (dist 450)
    while game.distance < 450.0 && game.state == GameState::Running {
        game.update(0.05);
    }

    assert!(matches!(game.state, GameState::Quiz { .. }));
    let stamina_before = game.player.stamina;

    // Answer incorrectly: Choice "B" is wrong
    let is_correct = game.answer_quiz("B");
    assert!(!is_correct);
    assert_eq!(game.last_audio_cue, AudioCue::QuizWrong);
    assert!(game.player.stamina < stamina_before);

    // Elapse feedback timer: player distance is pushed back slightly to retry
    game.update(2.1);
    assert_eq!(game.state, GameState::Running);
    assert!(game.distance < 450.0);
}

#[test]
fn test_full_game_simulation_to_victory() {
    let mut level = Level::default_episode_1();
    level.track_length = 500.0; // Shorter track for fast victory simulation
    level.checkpoints[0].distance = 250.0;
    level.checkpoints.truncate(1);

    let mut game = Game::new(level);
    game.start();

    // Run up to checkpoint
    while game.distance < 250.0 && game.state == GameState::Running {
        game.update(0.05);
    }
    assert!(matches!(game.state, GameState::Quiz { .. }));

    // Answer correctly
    game.answer_quiz("A");
    game.update(2.1);
    assert_eq!(game.state, GameState::Running);

    // Run to finish line
    while game.state == GameState::Running {
        game.update(0.05);
    }

    assert!(matches!(game.state, GameState::Victory { .. }));
    assert_eq!(game.last_audio_cue, AudioCue::Victory);
}
