use bevy::{
    app::{App, Plugin, Update},
    input::mouse::MouseWheel,
    prelude::*,
};
use bevy_magic_light_2d::SpriteCamera;

use crate::constants::{self};

pub struct CameraControlsPlugin;

impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_camera_movement, control_camera_zoom));
    }
}

fn control_camera_zoom(
    mut cameras: Query<&mut OrthographicProjection, With<SpriteCamera>>,
    time: Res<Time>,
    mut scroll_event_reader: EventReader<MouseWheel>,
) {
    let mut projection_delta = 0.;

    for event in scroll_event_reader.read() {
        projection_delta += event.y * 3.;
    }

    if projection_delta == 0. {
        return;
    }

    for mut camera in cameras.iter_mut() {
        camera.scale = (camera.scale - projection_delta * time.delta_seconds())
            .clamp(constants::camera::MIN_SCALE, constants::camera::MAX_SCALE);
    }
}

fn control_camera_movement(
    mut camera_current: Local<Vec2>,
    mut camera_target: Local<Vec2>,
    mut query_cameras: Query<&mut Transform, With<SpriteCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    
    if keyboard.pressed(KeyCode::KeyW) {
        camera_target.y += constants::camera::SPEED;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        camera_target.y -= constants::camera::SPEED;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        camera_target.x -= constants::camera::SPEED;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        camera_target.x += constants::camera::SPEED;
    }

    // Smooth camera.
    let blend_ratio = 0.2;
    let movement = *camera_target - *camera_current;
    *camera_current += movement * blend_ratio;

    // Update all sprite cameras.
    for mut camera_transform in query_cameras.iter_mut() {
        camera_transform.translation.x = camera_current.x;
        camera_transform.translation.y = camera_current.y;
    }
}
