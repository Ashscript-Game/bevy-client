use bevy::app::{App, Plugin};

use super::camera::CameraControlsPlugin;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraControlsPlugin);
    }
}