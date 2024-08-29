use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::hashbrown::HashMap,
};
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use rand::{thread_rng, Rng};

use crate::{
    components::{Laser, ResourceBlob},
    constants::{
        self, coal_node, laser, z_order, Resource, PROJECTILE_MOVE_END_TICK_PORTION,
        SECONDS_PER_TICK,
    },
    utils::{find_angle, find_angle_coords, signed_distance},
};

pub fn update_lasers(mut lasers: Query<(&mut Transform, &mut Laser)>, time: Res<Time>) {
    for (mut laser_transform, mut laser) in lasers.iter_mut() {
        /*         // the initial sign is important to detect which way we pass the target, negative or positive

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
        } */

        // translate the position of the blob to move linearly (relative x to y) towards the target
        // this should move the blob at a constant time of SECONDS_PER_TICK, no matter the distance or tick rate, it should reach the destination at the speed of the tick rate

        /* let direction = laser_transform.rotation * Vec3::Y;
        let distance = signed_distance(laser.start_pos, laser.target_pos);

        let translation_delta = distance / SECONDS_PER_TICK * time.delta_seconds() * direction;

        laser_transform.translation += translation_delta; */

        // laser_transform.rotation = Quat::from_rotation_z(angle);

        /* laser_transform.rotation.w += 0.01;
        let angle = laser_transform.rotation.w; */

        /* let target_angle = find_angle(&laser_transform.translation, &laser.target_pos);

         println!("target angle {}, laser angle {}", target_angle, laser.angle);

         if laser.angle > target_angle {
             laser.angle -= 0.1;
         }
         else {
             laser.angle += 0.1;
         }
        // laser.angle = target_angle;

         let angle = laser.angle;
         laser_transform.rotation = Quat::from_rotation_z(angle); */

        // use trig to apply evenly for diagonal vs straight movement

        let x_delta = (laser.target_pos.x - laser.start_pos.x) / SECONDS_PER_TICK / PROJECTILE_MOVE_END_TICK_PORTION * time.delta_seconds() /* * angle.cos().abs() */;
        let y_delta = (laser.target_pos.y - laser.start_pos.y) / SECONDS_PER_TICK / PROJECTILE_MOVE_END_TICK_PORTION * time.delta_seconds() /* * angle.sin().abs() */;

        laser_transform.translation.x += x_delta;
        laser_transform.translation.y += y_delta;
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

    let mut rng = thread_rng();
    let angle_offset = PI / rng.gen_range(0.1..=0.2);

    let angle = find_angle(start_pos, target_pos) /* + angle_offset */;

    let mesh = Mesh2dHandle(meshes.add(Circle::new(10.)));
    let color = laser::COLOR;

    let light = commands.spawn(PointLight2dBundle {
        transform: Transform::from_xyz(0., 0., 150.),
        point_light: PointLight2d {
            intensity: 0.5,
            color,
            radius: 100.,
            falloff: 10.,
            cast_shadows: true,
            // jitter_intensity: 0.1,
            // jitter_translation: 5.0,
            ..default()
        },
        ..default()
    }).id();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(start_pos.x, start_pos.y, z_order::PROJECTILE),
                /* rotation: Quat::from_rotation_z(angle), */
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        /* PointLight2d {
            intensity: 0.1,
            color,
            radius: 10.,
            falloff: 2.,
            cast_shadows: true,
            ..Default::default()
        }, */
        Laser {
            start_pos: *start_pos,
            target_pos: *target_pos,
            angle,
            damage,
        },
    )).add_child(light);
}

pub fn kill_lasers(mut commands: Commands, mut lasers: Query<(&Laser, Entity)>) {
    for (_, entity) in lasers.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
