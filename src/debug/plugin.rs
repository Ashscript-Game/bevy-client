use bevy::{
    app::{App, Plugin, Update},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_egui::{egui, EguiContexts};
use bevy_magic_light_2d::gi::render_layer::ALL_LAYERS;

use crate::components::{DebugSettings, FpsText};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_window);
    }
}

fn debug_window(
    mut egui: EguiContexts,
    mut debug_settings: ResMut<DebugSettings>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) else {
        return;
    };

    egui::Window::new("Debug")
        .anchor(egui::Align2::RIGHT_TOP, [0., 0.])
        .show(egui.ctx_mut(), |ui| {
            if let Some(value) = fps.smoothed() {
                ui.label(format!("FPS: {:.1}", value));
            }
            if let Some(value) = fps.average() {
                ui.label(format!("Avg FPS: {:.1}", value));
            }

            ui.checkbox(&mut debug_settings.hightlight_chunks, "Highlight chunks");
        });
}
