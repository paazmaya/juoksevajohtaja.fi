use juokseva_johtaja_game::level::{Choice, Level, QuestionCheckpoint};

#[test]
fn test_default_episode_1_loads_and_validates() {
    let level = Level::default_episode_1();
    assert_eq!(level.id, "1-1-undisclosed");
    assert!(level.track_length >= 1000.0);
    assert!(level.checkpoints.len() >= 3);
    assert!(level.obstacles.len() >= 2);
    assert!(level.pickups.len() >= 2);
    assert!(level.validate().is_ok());
}

#[test]
fn test_episode_template_validates() {
    let template_json = include_str!("../assets/levels/episode_template.json");
    let level = Level::from_json(template_json).expect("Template JSON must parse");
    assert!(level.validate().is_ok());
    assert_eq!(level.checkpoints[0].options.len(), 2);
    assert!(level.checkpoints[0].options.iter().any(|o| o.is_correct));
}

#[test]
fn test_level_validation_rejects_missing_correct_option() {
    let mut level = Level::default_episode_1();
    // Invalidate checkpoint options by setting all to false
    for opt in &mut level.checkpoints[0].options {
        opt.is_correct = false;
    }
    assert!(level.validate().is_err());
}

#[test]
fn test_level_validation_rejects_multiple_correct_options() {
    let mut level = Level::default_episode_1();
    for opt in &mut level.checkpoints[0].options {
        opt.is_correct = true;
    }
    assert!(level.validate().is_err());
}

#[test]
fn test_level_validation_rejects_out_of_bound_checkpoint() {
    let mut level = Level::default_episode_1();
    level.checkpoints.push(QuestionCheckpoint {
        id: "q_invalid".to_string(),
        distance: level.track_length + 500.0,
        question: "Out of bounds?".to_string(),
        options: vec![
            Choice {
                id: "A".to_string(),
                text: "Yes".to_string(),
                is_correct: true,
            },
            Choice {
                id: "B".to_string(),
                text: "No".to_string(),
                is_correct: false,
            },
        ],
        podcast_timestamp: None,
        explanation: None,
    });
    assert!(level.validate().is_err());
}
