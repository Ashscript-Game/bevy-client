
use ashscript_solis_2d::prelude::{Emitter, SdfShape};
use bevy::{
    app::{App, Plugin},
    prelude::*,
    render::view::RenderLayers,
    utils::{hashbrown::HashMap, HashSet},
};

use crate::{
    components::{Distributor, OccupiesTile, Store},
    constants::{self, distributor, Resource},
    engine::terrain::{HEX_LAYOUT, HEX_SIZE},
    utils::{self},
};


pub struct DistributorPlugin;

impl Plugin for DistributorPlugin {
    fn build(&self, _app: &mut App) {
        //app;
    }
}

pub fn spawn_distributor(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    let resource_options = vec![Resource::Coal, Resource::Minerals, Resource::Metal];
    let resource = utils::pick(&resource_options);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(distributor::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 5.0),
                scale: Vec3::new(0.08, 0.08, 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Distributor {
            resource: *resource,
            store: Store {
                resources: {
                    let mut map = HashMap::new();
                    map.insert(*resource, 1000);
                    map
                },
                allowed_inputs: Some(HashSet::from([*resource])),
                capacity: 1000,
            },
        },
        Emitter {
            intensity: 1.,
            color: constants::distributor::COLOR,
            shape: SdfShape::Circle(HEX_SIZE.x),
        },
    ));
}