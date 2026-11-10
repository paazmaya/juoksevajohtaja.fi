//! Separate obstacle entities and colliders in Bevy.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::level::LevelData;

pub const Z_OBSTACLE: f32 = 5.0;
pub const GROUND_Y: f32 = 420.0;

#[derive(Component)]
pub struct Obstacle {
    pub kind: String,
    pub width: f32,
    pub height: f32,
}

/// Spawns each obstacle from LevelData as a distinct entity with its own Sprite and Rapier Collider.
pub fn spawn_obstacles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<LevelData>,
) {
    for obs in &level.obstacles {
        let texture_path = if let Some(custom_img) = &obs.image {
            custom_img.clone()
        } else {
            match obs.kind.as_str() {
                "hurdle" => "obstacles/obstacle_hurdle.png".to_string(),
                "meeting" => "obstacles/obstacle_meeting.png".to_string(),
                _ => "obstacles/obstacle_cactus.png".to_string(),
            }
        };

        let texture_handle = asset_server.load(&texture_path);
        let x = obs.distance as f32;
        let y = GROUND_Y - (obs.height as f32) / 2.0;

        commands.spawn((
            Obstacle {
                kind: obs.kind.clone(),
                width: obs.width as f32,
                height: obs.height as f32,
            },
            SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(x, y, Z_OBSTACLE),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(obs.width as f32, obs.height as f32)),
                    ..default()
                },
                ..default()
            },
            // Physical sensor collider for jump / collision detection
            Collider::cuboid((obs.width as f32) / 2.0, (obs.height as f32) / 2.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}
