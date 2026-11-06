use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub id: String,
    pub title: String,
    pub podcast_url: String,
    pub track_length: f64,
    pub guest: CharacterConfig,
    pub host: CharacterConfig,
    pub checkpoints: Vec<QuestionCheckpoint>,
    pub obstacles: Vec<ObstacleData>,
    pub pickups: Vec<PickupData>,
    pub host_events: Vec<HostEventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterConfig {
    pub name: String,
    pub title: String,
    pub speed: f64,
    pub sprite: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QuestionCheckpoint {
    pub id: String,
    pub distance: f64,
    pub question: String,
    pub options: Vec<Choice>,
    pub podcast_timestamp: Option<String>,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub id: String,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObstacleData {
    pub distance: f64,
    pub kind: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PickupData {
    pub distance: f64,
    pub kind: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HostEventData {
    pub distance: f64,
    pub action: String, // "help" | "sabotage"
    pub dialogue: String,
}

impl Level {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Embedded default level for Season 1, Episode 1 ("Undisclosed")
    pub fn default_episode_1() -> Self {
        const DEFAULT_JSON: &str = include_str!("../assets/levels/episode_1_undisclosed.json");
        Self::from_json(DEFAULT_JSON).expect("Default episode 1 JSON must be valid")
    }

    /// Validate level integrity:
    /// - Every question checkpoint has at least 2 choices
    /// - Every checkpoint has exactly one correct choice
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
            if cp.options.len() < 2 {
                return Err(format!("Checkpoint {} has fewer than 2 choices", cp.id));
            }
            let correct_count = cp.options.iter().filter(|c| c.is_correct).count();
            if correct_count != 1 {
                return Err(format!(
                    "Checkpoint {} must have exactly 1 correct choice, found {}",
                    cp.id, correct_count
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
