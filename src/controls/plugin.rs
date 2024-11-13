use bevy::app::{App, Plugin, Update};

use super::{camera::CameraControlsPlugin, select::{handle_mouse_click, select_ui}};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraControlsPlugin)
            .add_systems(Update, (handle_mouse_click, select_ui));   
    }
}
