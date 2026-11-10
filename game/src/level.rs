//! Level definition, deserialization, and integrity validation for podcast running tracks.

use serde::{Deserialize, Serialize};

/// Minimum number of multiple-choice answers required per checkpoint.
pub const MIN_CHOICES_PER_CHECKPOINT: usize = 2;

/// Exact number of correct options required for each question checkpoint.
pub const REQUIRED_CORRECT_CHOICES: usize = 1;

/// Complete level configuration for a podcast episode track.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    /// Unique identifier slug for the episode (e.g. "1-1-undisclosed").
    pub id: String,
    /// Human-readable title of the episode level.
    pub title: String,
    /// Relative URL linking to the episode's podcast page.
    pub podcast_url: String,
    /// Total running track length in pixels/meters.
    pub track_length: f64,
    /// Three-layer background image paths for parallax rendering.
    #[serde(default)]
    pub background: Option<BackgroundConfig>,
    /// Character attributes for the guest runner (player).
    pub guest: CharacterConfig,
    /// Character attributes for the host runner (Jukka).
    pub host: CharacterConfig,
    /// Ordered list of question checkpoints along the track.
    pub checkpoints: Vec<QuestionCheckpoint>,
    /// Obstacles placed along the track that the player must jump over.
    pub obstacles: Vec<ObstacleData>,
    /// Collectible items placed along the track (coffee, podcast mics).
    pub pickups: Vec<PickupData>,
    /// Narrative events and dialogue triggered by the host at specific distances.
    pub host_events: Vec<HostEventData>,
}

/// Paths to the three parallax background layers for an episode level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundConfig {
    /// Far background layer asset path (sky, clouds, sun).
    pub layer1: String,
    /// Midground layer asset path (distant skyline, hills).
    pub layer2: String,
    /// Near foreground layer asset path (trees, lamps, roadside).
    pub layer3: String,
}

/// Character configuration defining name, title, running speed, and sprite identifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterConfig {
    /// Display name of the character.
    pub name: String,
    /// Subtitle or professional role shown on screen.
    pub title: String,
    /// Base horizontal running speed in pixels / second.
    pub speed: f64,
    /// Visual sprite asset identifier.
    pub sprite: String,
}

/// Checkpoint where the runner is asked a multiple-choice question from the podcast episode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QuestionCheckpoint {
    /// Unique checkpoint identifier (e.g. "q1").
    pub id: String,
    /// Distance from start in pixels where the checkpoint gate is located.
    pub distance: f64,
    /// Question text prompted to the player.
    pub question: String,
    /// Multiple-choice options.
    pub options: Vec<Choice>,
    /// Optional timestamp reference to the podcast audio (e.g. "08:15").
    pub podcast_timestamp: Option<String>,
    /// Educational explanation displayed when the player answers.
    pub explanation: Option<String>,
}

/// A multiple-choice option for a question checkpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    /// Option identifier (e.g. "A" or "B").
    pub id: String,
    /// Option text shown to the player.
    pub text: String,
    /// Whether this option is the correct answer.
    pub is_correct: bool,
}

/// Placement and dimensions for an obstacle on the running track.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObstacleData {
    /// Distance along the track in pixels.
    pub distance: f64,
    /// Obstacle type ("hurdle", "meeting", "cactus").
    pub kind: String,
    /// Width of the obstacle bounding box in pixels.
    pub width: f64,
    /// Height of the obstacle bounding box in pixels.
    pub height: f64,
    /// Optional custom image asset path for this obstacle.
    #[serde(default)]
    pub image: Option<String>,
}

/// Collectible pickup item placed along the track.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PickupData {
    /// Distance along the track in pixels.
    pub distance: f64,
    /// Type of pickup ("coffee", "podcast_mic").
    pub kind: String,
    /// Stamina replenishment value granted upon collection.
    pub value: f64,
}

/// Narrative dialogue event triggered when runners reach a specific distance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HostEventData {
    /// Distance along the track in pixels where the dialogue triggers.
    pub distance: f64,
    /// Host action ("help" | "sabotage").
    pub action: String,
    /// Dialogue text spoken by the host in a speech bubble.
    pub dialogue: String,
}

impl Level {
    /// Deserializes a `Level` from a JSON string.
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Embedded default level for Season 1, Episode 1 ("Undisclosed").
    pub fn default_episode_1() -> Self {
        const DEFAULT_JSON: &str = include_str!("../assets/levels/episode_1_undisclosed.json");
        Self::from_json(DEFAULT_JSON).expect("Default episode 1 JSON must be valid")
    }

    /// Validate level integrity:
    /// - Track length must be positive
    /// - Every question checkpoint has at least `MIN_CHOICES_PER_CHECKPOINT` choices
    /// - Every checkpoint has exactly `REQUIRED_CORRECT_CHOICES` correct choice
    /// - Checkpoints and obstacles are within track length
    pub fn validate(&self) -> Result<(), String> {
        if self.track_length <= 0.0 {
            return Err("track_length must be positive".to_string());
        }

        for cp in &self.checkpoints {
            if cp.distance < 0.0 || cp.distance > self.track_length {
                return Err(format!(
                    "Checkpoint {} distance {} is out of bounds",
                    cp.id, cp.distance
                ));
            }
            if cp.options.len() < MIN_CHOICES_PER_CHECKPOINT {
                return Err(format!(
                    "Checkpoint {} has fewer than {} choices",
                    cp.id, MIN_CHOICES_PER_CHECKPOINT
                ));
            }
            let correct_count = cp.options.iter().filter(|c| c.is_correct).count();
            if correct_count != REQUIRED_CORRECT_CHOICES {
                return Err(format!(
                    "Checkpoint {} must have exactly {} correct choice, found {}",
                    cp.id, REQUIRED_CORRECT_CHOICES, correct_count
                ));
            }
        }

        for obs in &self.obstacles {
            if obs.distance < 0.0 || obs.distance > self.track_length {
                return Err(format!(
                    "Obstacle at {} is out of track bounds",
                    obs.distance
                ));
            }
        }

        Ok(())
    }
}
