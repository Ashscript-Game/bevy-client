
use ashscript_types::constants::map::HEX_LAYOUT;
use bevy::{
    app::{App, Plugin},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};
use uuid::Uuid;

use crate::{
    components::{OccupiesTile, Owner, Turret},
    constants::{self, turret},
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
    owner_id: Uuid,
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
            ..default()
        },
        OmniLightSource2D {
            intensity: 0.2,
            color: constants::turret::COLOR,
            falloff: Vec3::new(4., 4., 0.005),
            ..default()
        },
        Owner(owner_id),
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}