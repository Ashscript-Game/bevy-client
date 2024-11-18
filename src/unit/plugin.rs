use ashscript_types::{components::{body::UnitBody, energy::Energy, health::Health}, constants::map::HEX_LAYOUT, objects::GameObjectKind};
use bevy::{
    app::{App, Plugin},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};
use enum_map::enum_map;
use hexx::Hex;
use rand;
use uuid::Uuid;

use crate::{
    components::{EnergyComp, GameObjectKindComp, HealthComp, MappedGameObjects, OccupiesTile, Owner, Unit, UnitBodyComp},
    constants::{unit, UnitPart},
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, _app: &mut App) {
        /* app.add_plugins(UnitBenchmarks); */
    }
}

pub fn create_unit(
    hex: Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_object_map: &mut MappedGameObjects,
    health: Health,
    owner_id: Uuid,
    body: UnitBody,
    energy: Energy,
) {

    /* let mesh = Mesh2dHandle(meshes.add(Circle::new(30.)));
    let color = unit::COLOR; */

    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    let entity = commands
        .spawn((
            /* MaterialMesh2dBundle {
                mesh,
                material: materials.add(color),
                transform: Transform {
                    translation: Vec3::new(
                        world_pos.x,
                        world_pos.y,
                        1.,
                    ),
                    // rotation: Quat::from_rotation_z(angle),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            }, */
            SpriteBundle {
                texture: asset_server.load(unit::ASSET_PATH),
                transform: Transform {
                    translation: Vec3::new(world_pos.x, world_pos.y, 1.0),
                    scale: Vec3::new(1.2, 1.2, 1.0),
                    ..default()
                },
                ..default()
            },
            OmniLightSource2D {
                intensity: 0.1,
                color: unit::LIGHT_COLOR,
                falloff: Vec3::new(2., 2., 0.005),
                ..Default::default()
            },
            EnergyComp(energy),
            OccupiesTile,
            HealthComp(health),
            Unit {
                ..default()
            },
            UnitBodyComp(body),
            GameObjectKindComp(GameObjectKind::Unit),
            Owner(owner_id),
            RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
        ))
        .id();

    game_object_map.insert(hex, GameObjectKind::Unit, entity);
}
