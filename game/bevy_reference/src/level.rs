//! Level and obstacle deserialization for Bevy from episode JSON files.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelData {
    pub id: String,
    pub title: String,
    pub podcast_url: String,
    pub track_length: f64,
    #[serde(default)]
    pub background: Option<BackgroundConfig>,
    pub guest: CharacterConfig,
    pub host: CharacterConfig,
    pub checkpoints: Vec<QuestionCheckpoint>,
    pub obstacles: Vec<ObstacleData>,
    pub pickups: Vec<PickupData>,
    pub host_events: Vec<HostEventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundConfig {
    pub layer1: String,
    pub layer2: String,
    pub layer3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterConfig {
    pub name: String,
    pub title: String,
    pub speed: f64,
    pub sprite: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionCheckpoint {
    pub id: String,
    pub distance: f64,
    pub question: String,
    pub options: Vec<Choice>,
    pub podcast_timestamp: Option<String>,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub id: String,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObstacleData {
    pub distance: f64,
    pub kind: String,
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PickupData {
    pub distance: f64,
    pub kind: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostEventData {
    pub distance: f64,
    pub action: String,
    pub dialogue: String,
}

impl LevelData {
    pub fn load_default() -> Self {
        const DEFAULT_JSON: &str = include_str!("../../assets/levels/episode_1_undisclosed.json");
        serde_json::from_str(DEFAULT_JSON).expect("Default episode 1 JSON must parse")
    }
}
