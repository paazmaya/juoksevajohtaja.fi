//! Character sprite sheets, frame animation, and runner physics in Bevy.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const Z_RUNNER: f32 = 10.0;
pub const RUNNER_FRAME_SIZE: UVec2 = UVec2::new(96, 96);
pub const RUNNER_FRAME_COUNT: usize = 5;

#[derive(Component)]
pub struct Runner {
    pub speed: f32,
    pub stamina: f32,
    pub is_player: bool,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

/// Spawns the guest runner and host companion using Bevy's TextureAtlasLayout.
pub fn spawn_runners(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 5 frames in a 1x5 strip, 96x96 px per frame
    let layout = TextureAtlasLayout::from_grid(RUNNER_FRAME_SIZE, RUNNER_FRAME_COUNT as u32, 1, None, None);
    let atlas_layout = texture_atlas_layouts.add(layout);

    // 1. Player (Guest Runner - Adventurer)
    let player_texture = asset_server.load("characters/adventurer/spritesheet.png");
    commands.spawn((
        Runner {
            speed: 100.0,
            stamina: 100.0,
            is_player: true,
        },
        SpriteSheetBundle {
            texture: player_texture,
            atlas: TextureAtlas {
                layout: atlas_layout.clone(),
                index: 0, // Frame 0: Idle
            },
            transform: Transform::from_xyz(120.0, 385.0, Z_RUNNER),
            ..default()
        },
        AnimationIndices { first: 1, last: 2 }, // Walk frames 1 and 2
        AnimationTimer(Timer::from_seconds(0.16, TimerMode::Repeating)),
        RigidBody::Dynamic,
        Collider::cuboid(17.0, 35.0),
        LockedAxes::ROTATION_LOCKED,
    ));

    // 2. Host Companion (Jukka Paasonen)
    let host_texture = asset_server.load("characters/player/spritesheet.png");
    commands.spawn((
        Runner {
            speed: 98.0,
            stamina: 100.0,
            is_player: false,
        },
        SpriteSheetBundle {
            texture: host_texture,
            atlas: TextureAtlas {
                layout: atlas_layout,
                index: 0,
            },
            transform: Transform::from_xyz(190.0, 385.0, Z_RUNNER - 0.1),
            ..default()
        },
        AnimationIndices { first: 1, last: 2 },
        AnimationTimer(Timer::from_seconds(0.16, TimerMode::Repeating)),
        RigidBody::Dynamic,
        Collider::cuboid(17.0, 35.0),
        LockedAxes::ROTATION_LOCKED,
    ));
}

/// System that steps through active animation frames using the frame timer.
pub fn animate_sprites(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas, &Runner)>,
) {
    let is_moving = keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD);

    for (indices, mut timer, mut atlas, runner) in &mut query {
        if runner.is_player && !is_moving {
            // Idle frame when standing still
            atlas.index = 0;
            continue;
        }

        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            atlas.index = if atlas.index >= indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
