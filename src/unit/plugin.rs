use bevy::{app::{App, Plugin}, prelude::*, render::view::RenderLayers, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_light_2d::light::PointLight2d;
use enum_map::enum_map;
use hexx::Hex;

use crate::{components::{OccupiesTile, Unit}, constants::{unit, z_order, UnitPart}, engine::terrain::HEX_LAYOUT};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        /* app.add_plugins(UnitBenchmarks); */
    }
}

pub fn spawn_test_unit() {


}

pub fn spawn_unit(
    hex: Hex,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {

    let mesh = Mesh2dHandle(meshes.add(Circle::new(30.)));
    let color = unit::COLOR;

    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(
                    world_pos.x,
                    world_pos.y,
                    1.,
                ),
                /* rotation: Quat::from_rotation_z(angle), */
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        PointLight2d {
            intensity: 0.1,
            color: unit::LIGHT_COLOR,
            falloff: 2.,
            cast_shadows: true,
            ..Default::default()
        },
        OccupiesTile,
        Unit {
            health: 100,
            body: enum_map! {
                UnitPart::Ranged => 3,
                UnitPart::Generate => 6,
                _ => 1,
            },
            ..default()
        },
    ));
}