use bevy::{
    app::{App, Plugin, Update},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::view::RenderLayers,
};

use crate::components::FpsText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_fps_text)
            .add_systems(Update, update_fps_text);
    }
}

fn create_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load("fonts/outfit.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(
                // "default_font" feature is unavailable, load a font to use instead.
                TextStyle {
                    font: asset_server.load("fonts/outfit.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                },
            ),
        ]),
        FpsText,
        RenderLayers::all(),
    ));
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
