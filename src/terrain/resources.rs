use bevy::{app::{App, Plugin, Startup, Update}, math::Vec3, prelude::*};
use hexx::hex;

use crate::{components::ResourceNode, constants::resource_node, terrain::tiles::HEX_LAYOUT};

use super::tiles::TilePlugin;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_nodes, generate_scrap));
    }
}

fn generate_nodes(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("generating nodes");

    let point = hex(8, -12);
    let world_pos = HEX_LAYOUT.hex_to_world_pos(point);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(resource_node::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, resource_node::Z_POS),
                ..default()
            },
            ..default()
        },
        ResourceNode {
            coal_percent: 50,
            mineral_percent: 50,
            ticks_to_regen: 0,
            resource_remaining: 1000,
        },
    ));
}

fn generate_scrap() {

}