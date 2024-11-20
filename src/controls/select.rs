use ashscript_types::{constants::map::HEX_LAYOUT, objects::GameObjectKind};
use bevy::{ecs::observer::TriggerTargets, input::mouse::MouseButtonInput, prelude::*};
use bevy_egui::{
    egui::{self, Color32, Vec2, Vec2b},
    EguiContexts,
};

use crate::{
    components::{
        EnergyComp, GameObjectKindComp, HealthComp, MappedGameObjects, Owner, Selected, SelectedGameObjects, StorageComp, Unit, UnitBodyComp
    },
    ui::{
        constants::{spacing, text_size, LATTE},
        theme::set_theme,
        widgets::{custom_header, header, header_with_text},
    },
};

pub fn handle_mouse_click(
    mut mouse_reader: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    units: Query<(&GameObjectKindComp, &Transform)>,
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
                        let (_, transform) = units.get(*entity).unwrap();

                        println!("selected unit at: {} {}", hex.x, hex.y);

                        let hex = HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate());

                        selected.0.insert(*entity);
                    }

                    println!("mouse pos: {}", mouse_pos);
                }
                MouseButton::Right => {
                    println!("right mouse button clicked");

                    selected.0.clear();
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
        Option<&GameObjectKindComp>,
        Option<&Owner>,
        Option<&Unit>,
        Option<&UnitBodyComp>,
        Option<&HealthComp>,
        Option<&EnergyComp>,
        Option<&StorageComp>,
    )>,
) {
    let panel = egui::SidePanel::right("select")
        .min_width(200.0)
        .max_width(200.0);

    set_theme(egui.ctx_mut(), LATTE);

    panel.show(egui.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for entity in selected.0.iter() {
                let Ok((transform, kind, owner, unit, unit_body, health, energy, storage)) = query.get(*entity)
                else {
                    continue;
                };

                ui.add_space(spacing::XSMALL);

                if let Some(kind) = kind {
                    custom_header(ui, format!("{:?}", kind.0), text_size::MEDIUM);
                    ui.add_space(spacing::XSMALL);
                }

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

                ui.add_space(spacing::XSMALL);

                if let Some(owner) = owner {
                    header_with_text(ui, "Owner:", owner.0);
                    ui.add_space(spacing::XSMALL);
                }

                if let Some(health) = health {
                    header_with_text(
                        ui,
                        "Health:",
                        format!("{} / {}", health.0.current, health.0.max),
                    );
                    ui.add_space(spacing::XSMALL);
                }

                if let Some(energy) = energy {
                    header_with_text(
                        ui,
                        "Energy:",
                        format!("{} / {}", energy.0.current, energy.0.capacity),
                    );
                    ui.add_space(spacing::XSMALL);
                }

                if let Some(unit_body) = unit_body {
                    header_with_text(
                        ui,
                        "Age",
                        format!("{} / {}", unit_body.0.age, unit_body.0.max_age()),
                    );
                    ui.add_space(spacing::XSMALL);

                    header(ui, "Unit body");
                    ui.add_space(spacing::XSMALL);

                    ui.vertical(|ui| {
                        for (part, count) in unit_body.0.parts.iter() {
                            if *count == 0 {
                                continue;
                            }

                            header_with_text(ui, format!("{:?}:", part), count.to_string());
                            ui.add_space(spacing::XSMALL);
                        }
                    });
                }

                if let Some(storage) = storage {
                    header(ui, "Storage");
                    ui.add_space(spacing::XSMALL);

                    ui.vertical(|ui| {
                        for (resource, count) in storage.0.resources.iter() {
                            header_with_text(ui, format!("{:?}:", resource), count.to_string());
                            ui.add_space(spacing::XSMALL);
                        }
                    });
                }

                ui.separator();
            }
        });
    });
}
