//! Three-layer parallax background setup and camera tracking in Bevy.

use bevy::prelude::*;
use bevy_parallax::{ParallaxCameraComponent, ParallaxLayer, ParallaxResource};
use crate::level::LevelData;

pub const Z_SKY: f32 = -30.0;
pub const Z_MIDGROUND: f32 = -20.0;
pub const Z_FOREGROUND: f32 = -10.0;

/// Initializes the 2D camera and registers the three background layers.
pub fn setup_parallax_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<LevelData>,
) {
    // 1. Spawn 2D camera with parallax tracking
    commands.spawn((
        Camera2dBundle::default(),
        ParallaxCameraComponent::default(),
    ));

    // 2. Determine paths from level data or fall back to defaults
    let (l1, l2, l3) = if let Some(bg) = &level.background {
        (bg.layer1.clone(), bg.layer2.clone(), bg.layer3.clone())
    } else {
        (
            "backgrounds/helsinki/layer1_sky.png".to_string(),
            "backgrounds/helsinki/layer2_midground.png".to_string(),
            "backgrounds/helsinki/layer3_foreground.png".to_string(),
        )
    };

    // 3. Register the 3 layers in Bevy ParallaxResource
    commands.insert_resource(ParallaxResource {
        layer_data: vec![
            // Layer 1: Distant sky and sun (slowest)
            ParallaxLayer {
                texture: asset_server.load(&l1),
                speed: 0.05,
                repeat_x: true,
                z: Z_SKY,
                ..default()
            },
            // Layer 2: Midground hills & skyline
            ParallaxLayer {
                texture: asset_server.load(&l2),
                speed: 0.25,
                repeat_x: true,
                z: Z_MIDGROUND,
                ..default()
            },
            // Layer 3: Foreground roadside scenery & trees
            ParallaxLayer {
                texture: asset_server.load(&l3),
                speed: 0.65,
                repeat_x: true,
                z: Z_FOREGROUND,
                ..default()
            },
        ],
        ..default()
    });
}
