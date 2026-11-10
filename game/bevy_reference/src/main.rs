//! Bevy reference implementation for Juokseva Johtaja 2D running game.

mod character;
mod level;
mod obstacles;
mod parallax;

use bevy::prelude::*;
use bevy_parallax::ParallaxPlugin;
use bevy_rapier2d::prelude::*;
use character::{animate_sprites, spawn_runners};
use level::LevelData;
use obstacles::spawn_obstacles;
use parallax::setup_parallax_background;

fn main() {
    let level_data = LevelData::load_default();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Juokseva Johtaja - Bevy Runner".to_string(),
                resolution: (960.0, 540.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ParallaxPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(level_data)
        .add_systems(Startup, (setup_parallax_background, spawn_runners, spawn_obstacles))
        .add_systems(Update, animate_sprites)
        .run();
}
