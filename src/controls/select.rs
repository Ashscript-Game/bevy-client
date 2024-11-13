use ashscript_types::{constants::map::HEX_LAYOUT, objects::GameObjectKind};
use bevy::{ecs::observer::TriggerTargets, input::mouse::MouseButtonInput, prelude::*};
use bevy_egui::{egui, EguiContexts};

use crate::components::{Health, MappedGameObjects, Selected, SelectedGameObjects, Unit, UnitPartComp};

pub fn handle_mouse_click(
    mut mouse_reader: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    units: Query<(&Unit, &Transform)>,
    mapped_game_objects: MappedGameObjects,
    mut selected: ResMut<SelectedGameObjects>,
) {
    let window = windows.single();

    for (camera, camera_transform) in cameras.iter() {
        for event in mouse_reader.read() {
            match event.button {
                MouseButton::Left => {
                    println!("left mouse button clicked");
                    event.state.is_pressed();

                    let Some(mouse_pos) = window
                        .cursor_position()
                        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
                    else {
                        continue;
                    };

                    let hex = HEX_LAYOUT.world_pos_to_hex(mouse_pos);

                    if let Some(entity) = mapped_game_objects.entity(&hex, GameObjectKind::Unit) {
                        let (unit, transform) = units.get(*entity).unwrap();

                        println!("selected unit at: {} {}", hex.x, hex.y);

                        let hex = HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate());

                        selected.0.insert(*entity);
                    }

                    println!("mouse pos: {}", mouse_pos);
                }
                MouseButton::Right => {
                    println!("right mouse button clicked");
                }
                _ => {}
            }
        }
    }
}

pub fn select_ui(
    selected: ResMut<SelectedGameObjects>,
    mut egui: EguiContexts,
    query: Query<(&Transform, Option<&Unit>, Option<&UnitPartComp>, Option<&Health>)>,
) {
    let panel = egui::SidePanel::right("select").min_width(200.0);

    panel.show(egui.ctx_mut(), |ui| {
        for entity in selected.0.iter() {
            let Ok((transform, unit, unit_body, health)) = query.get(*entity) else {
                continue;
            };

            ui.label(format!(
                "Position: {}, {}",
                HEX_LAYOUT
                    .world_pos_to_hex(transform.translation.truncate())
                    .x,
                HEX_LAYOUT
                    .world_pos_to_hex(transform.translation.truncate())
                    .y
            ));

            if let Some(health) = health {
                ui.label(format!("Health: {} / {}", health.current, health.max));
            }


        }
    });
}
