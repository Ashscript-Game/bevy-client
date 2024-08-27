use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::hashbrown::HashMap,
};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};

use crate::{
    components::{Laser, ResourceBlob},
    constants::{self, coal_node, laser, z_order, Resource, SECONDS_PER_TICK},
    utils::{find_angle, signed_distance},
};

pub fn update_lasers(
    mut lasers: Query<(&mut Transform, &Laser, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {

    for (mut laser_transform, laser, entity) in lasers.iter_mut() {

        // the initial sign is important to detect which way we pass the target, negative or positive 

        let horizontal_sign = (laser.target_pos.x - laser.start_pos.x).signum();
        let vertical_sign = (laser.target_pos.y - laser.start_pos.y).signum();
        
        // if we have passed or reached the target, despawn the blob
        // && or || might not work as it might move along one axis vert / horiz
        // changed from >= to > to account for above
        if (laser_transform.translation.x - laser.target_pos.x) * horizontal_sign > 0.
            || (laser_transform.translation.y - laser.target_pos.y) * vertical_sign > 0.
        {
            /* println!("despawning resource blob {:?}", blob.resource); */
            commands.entity(entity).despawn();
            continue;
        }

        // translate the position of the blob to move linearly (relative x to y) towards the target
        // this should move the blob at a constant time of SECONDS_PER_TICK, no matter the distance or tick rate, it should reach the destination at the speed of the tick rate

        let direction = laser_transform.rotation * Vec3::Y;
        let distance = signed_distance(laser.start_pos, laser.target_pos);

        let translation_delta = distance / SECONDS_PER_TICK * time.delta_seconds() * direction;

        laser_transform.translation += translation_delta;

        // use trig to apply evenly for diagonal vs straight movement

        /* let x_delta = (blob.target_pos.x - blob.start_pos.x) / SECONDS_PER_TICK * time.delta_seconds() * direction.x;
        let y_delta = (blob.target_pos.y - blob.start_pos.y) / SECONDS_PER_TICK * time.delta_seconds() * direction.y;

        blob_transform.translation.x += x_delta;
        blob_transform.translation.y += y_delta; */
    }
}

pub fn create_laser(
    start_pos: &Vec3,
    target_pos: &Vec3,
    damage: u32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    /* println!("creating resource blob {:?}", resource); */

    let angle = find_angle(start_pos.x, start_pos.y, target_pos.x, target_pos.y) + PI / 2.;

    let mesh = Mesh2dHandle(meshes.add(Circle::new(10.)));
    let color = laser::COLOR;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(start_pos.x, start_pos.y, z_order::PROJECTILE),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            ..default()
        },
        OmniLightSource2D {
            intensity: 0.1,
            color,
            falloff: Vec3::new(2., 2., 0.005),
            ..Default::default()
        },
        Laser {
            start_pos: *start_pos,
            target_pos: *target_pos,
            damage,
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}
