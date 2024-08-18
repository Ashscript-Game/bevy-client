use bevy::{app::{App, Plugin, Update}, input::mouse::MouseWheel, prelude::*};

use crate::constants::{self, control_keys, ResultCode};

pub struct CameraControlsPlugin;

impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_camera_movement, control_camera_zoom));
    }
}

fn control_camera_zoom(
    mut cameras: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
    mut scroll_event_reader: EventReader<MouseWheel>,
) {
    let mut projection = cameras.single_mut();

    for event in scroll_event_reader.read() {
        let projection_delta = event.y * 3.;

        projection.scale -= projection_delta * time.delta_seconds();
    }
}

fn control_camera_movement(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {

    let Ok((_, mut camera_transform)) = camera_query.get_single_mut() else {
        return;
    };

    let translation = find_camera_translation(&input, &time);
    let Ok(translation) = translation else { return };

    apply_camera_translation(translation, &mut camera_transform);
}

fn find_camera_translation(
    input: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
) -> Result<Vec2, ResultCode> {
    let speed = find_speed(input);
    let mut translation = Vec2::new(0., 0.);

    // Replace this with a match expression later

    if input.pressed(control_keys::MOVE_UP) {
        translation.y += speed;
    }

    if input.pressed(control_keys::MOVE_DOWN) {
        translation.y -= speed;
    }

    if input.pressed(control_keys::MOVE_LEFT) {
        translation.x -= speed;
    }

    if input.pressed(control_keys::MOVE_RIGHT) {
        translation.x += speed;
    }

    if translation.x == 0. && translation.y == 0. {
        return Err(ResultCode::NoAction);
    };

    let delta_seconds = time.delta_seconds();
    translation.x *= delta_seconds;
    translation.y *= delta_seconds;

    Ok(translation)
}

fn apply_camera_translation(translation: Vec2, camera_transform: &mut Transform) {
    
    camera_transform.translation = translation.extend(camera_transform.translation.z);
}

fn find_speed(input: &Res<ButtonInput<KeyCode>>) -> f32 {
    if input.pressed(control_keys::BOOST) {
        return constants::camera::BOOST_SPEED;
    }

    // Otherwise we aren't boosting

    constants::camera::SPEED
}