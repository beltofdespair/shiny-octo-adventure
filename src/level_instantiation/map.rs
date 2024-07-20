use std::time::Duration;

use crate::{
    file_system_interaction::asset_loading::GltfAssets,
    player_control::{
        actions::create_camera_action_input_manager_bundle,
        camera::IngameCamera,
    },
    GameState,
};
use bevy::{gltf::Gltf, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_dolly::prelude::*;

// without this the camera spawns before other things
const WAIT_TO_SPAWN_CAMERA: f32 = 6.0;

#[derive(Component)]
struct SpawnTimer(Timer);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_level)
        .add_systems(Startup, camera_spawn_timer)
        .add_systems(Update, tick_timer.run_if(in_state(GameState::Playing)))
        .add_systems(Update, spawn_camera.run_if(in_state(GameState::Playing)));
}

fn spawn_level(
    mut commands: Commands,
    models: Res<Assets<Gltf>>,
    gltf_assets: Res<GltfAssets>,
) {
    let gltf = models.get(&gltf_assets.level).unwrap();
    commands.spawn((
        SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..default()
        },
        Name::new("Level"),
    ));
}

fn camera_spawn_timer(mut commands: Commands) {
    commands.spawn(SpawnTimer(Timer::from_seconds(
        WAIT_TO_SPAWN_CAMERA,
        TimerMode::Once,
    )));
}

fn tick_timer(mut query: Query<&mut SpawnTimer>) {
    query.single_mut().0.tick(Duration::from_secs_f32(1.0));
}

fn spawn_camera(mut commands: Commands, query: Query<&SpawnTimer>) {
    for timer in query.iter() {
        if timer.0.just_finished() {
            commands.spawn((
                Name::new("Camera"),
                Camera3dBundle::default(),
                IngameCamera::default(),
                AtmosphereCamera::default(),
                IsDefaultUiCamera,
                Rig::builder()
                    .with(Position::default())
                    .with(YawPitch::default())
                    .with(Smooth::default())
                    .with(Arm::new(default()))
                    .with(LookAt::new(default()).tracking_predictive(true))
                    .build(),
                create_camera_action_input_manager_bundle(),
            ));
        }
    }
}
