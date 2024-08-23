use bevy::{
    app::{App, Plugin, Startup}, prelude::*, render::view::RenderLayers, utils::hashbrown::HashMap
};
use bevy_magic_light_2d::prelude::CAMERA_LAYER_OBJECTS;
use hexx::{hex, shapes, Hex};

use crate::{
    components::{Assembler, OccupiesTile, Structure},
    constants::{self, assembler},
    terrain::tiles::HEX_LAYOUT,
};

pub struct AssemblerPlugin;

impl Plugin for AssemblerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_assemblers);
    }
}

fn generate_assemblers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<OccupiesTile>>,
) {

    for hex in shapes::hexagon(hex(8, -6), 2) {

        spawn_assembler(hex, &mut commands, &asset_server, &query);
    }
}

fn spawn_assembler(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    query: &Query<&Transform, With<OccupiesTile>>,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    // not very efficient
    for transform in query.iter() {
        if transform.translation.truncate() == world_pos {
            return;
        }
    }

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(assembler::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 1.0),
                scale: Vec3::new(1.2, 1.2, 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Structure,
        Assembler {
            output_resource: constants::Resource::Metal,
            store: HashMap::new(),
            transferring: None
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS)
    ));
}
