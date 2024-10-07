
use bevy::{
    prelude::*,
    render::view::RenderLayers,
    utils::{hashbrown::HashMap, HashSet},
};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};

use crate::{
    components::{Factory, OccupiesTile, Store},
    constants::{factory, Resource},
    engine::terrain::HEX_LAYOUT,
};

pub fn spawn_factory(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    owner_id: u32,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(factory::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 5.0),
                scale: Vec3::new(1., 1., 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Factory {
            production_progress: 100,
            store: Store {
                resources: {
                    let mut map = HashMap::new();
                    map.insert(Resource::Metal, 1000);
                    map
                },
                allowed_inputs: Some(HashSet::from([Resource::Metal])),
                capacity: 1000,
            },
            owner_id,
            energy: 100,
            energy_capacity: 1000
        },
        OmniLightSource2D {
            intensity: 0.2,
            color: factory::COLOR,
            falloff: Vec3::new(4., 4., 0.005),
            ..default()
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}