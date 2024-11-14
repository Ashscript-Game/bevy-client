use ashscript_types::{constants::map::HEX_LAYOUT, objects::GameObjectKind};
use bevy::{ecs::observer::TriggerTargets, input::mouse::MouseButtonInput, prelude::*};
use bevy_egui::{
    egui::{self, Color32, Vec2b},
    EguiContexts,
};

use crate::{
    components::{
        Health, MappedGameObjects, Owner, Selected, SelectedGameObjects, Unit, UnitBodyComp,
    },
    ui::{
        constants::{spacing, LATTE},
        theme::set_theme,
        widgets::{header, header_with_text},
    },
};

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
    query: Query<(
        &Transform,
        Option<&Owner>,
        Option<&Unit>,
        Option<&UnitBodyComp>,
        Option<&Health>,
    )>,
) {
    let panel = egui::SidePanel::right("select").min_width(200.0);

    set_theme(egui.ctx_mut(), LATTE);

    panel.show(egui.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for entity in selected.0.iter() {
                let Ok((transform, owner, unit, unit_body, health)) = query.get(*entity) else {
                    continue;
                };

                ui.add_space(spacing::SMALL);

                header_with_text(
                    ui,
                    "Position:",
                    format!(
                        "{}, {}",
                        HEX_LAYOUT
                            .world_pos_to_hex(transform.translation.truncate())
                            .x,
                        HEX_LAYOUT
                            .world_pos_to_hex(transform.translation.truncate())
                            .y
                    ),
                );

                ui.add_space(spacing::SMALL);

                if let Some(owner) = owner {
                    header_with_text(ui, "Owner:", owner.0);
                    // ui.label(format!("Owner: {}", owner.0)).color(Color32::WHITE);
                    ui.add_space(spacing::SMALL);
                }

                if let Some(health) = health {
                    header_with_text(
                        ui,
                        "Health:",
                        format!("{} / {}", health.current, health.max),
                    );
                    ui.add_space(spacing::SMALL);
                }

                if let Some(unit_body) = unit_body {
                    header(ui, "Unit body");
                    ui.add_space(spacing::SMALL);

                    for (part, count) in unit_body.0.parts.iter() {
                        ui.label(format!("{:?}: {}", part, count));
                        header_with_text(
                            ui,
                            format!("{:?}:", part),
                            format!("{:?}: {}", part, count),
                        );
                        ui.add_space(spacing::SMALL);
                    }
                }

                ui.separator();
            }
        });
    });
}
