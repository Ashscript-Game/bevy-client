
use ashscript_solis_2d::prelude::{Emitter, SdfShape};
use bevy::{
    app::{App, Plugin},
    prelude::*,
    render::view::RenderLayers,
};

use crate::{
    components::{OccupiesTile, Turret},
    constants::{self, turret},
    engine::terrain::HEX_LAYOUT,
};


pub struct DistributorPlugin;

impl Plugin for DistributorPlugin {
    fn build(&self, _app: &mut App) {
        //app;
    }
}

pub fn spawn_turret(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(turret::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 5.0),
                scale: Vec3::new(1., 1., 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Turret {
            range: 6,
            damage: 2,
            energy_gen: 60,
            ..default()
        },
        Emitter {
            intensity: 1.,
            color: constants::turret::COLOR,
            shape: SdfShape::Circle(200.),
        },
    ));
}