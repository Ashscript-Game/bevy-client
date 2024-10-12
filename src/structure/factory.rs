
use ashscript_solis_2d::prelude::{Emitter, SdfShape};
use bevy::{
    prelude::*,
    render::view::RenderLayers,
    utils::{hashbrown::HashMap, HashSet},
};

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
        Emitter {
            intensity: 1.,
            color: factory::COLOR,
            shape: SdfShape::Circle(200.),
        },
    ));
}