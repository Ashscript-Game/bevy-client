use core::f32;
use std::f32::consts::PI;

use bevy::{
    ecs::observer::TriggerTargets,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::hashbrown::HashMap,
};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};
use rand::{thread_rng, Rng};

use crate::{
    components::{Laser, ResourceBlob, Unit},
    constants::{
        self, coal_node, laser, projectile, z_order, Resource, PROJECTILE_MOVE_END_TICK_PORTION,
        SECONDS_PER_TICK,
    },
    utils::{find_angle, find_angle_coords, signed_distance},
};

pub fn update_lasers(
    mut lasers: Query<(&mut Transform, &mut Laser, Entity), Without<Unit>>,
    units: Query<(&Unit, &Transform), Without<Laser>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut laser_transform, mut laser, laser_entity) in lasers.iter_mut() {
        let Ok((unit, unit_transform)) = units.get(laser.target_entity) else {
            commands.entity(laser_entity).despawn();
            continue;
        };

        let laser_aabb = Aabb2d::new(
            laser_transform.translation.truncate(),
            laser_transform.scale.truncate() / 2.,
        );

        let unit_aabb = Aabb2d::new(
            unit_transform.translation.truncate(),
            unit_transform.scale.truncate() / 2.,
        );

        if laser_aabb.intersects(&unit_aabb) {
            println!("intersection");
            commands.entity(laser_entity).despawn();
            continue;
        }

        // check if the unit intersects with the laser

        /* if laser_transform.translation.x < unit_transform.translation.x - unit.shape.width / 2.
            || laser_transform.translation.x > unit_transform.translation.x + unit.shape.width / 2.
            || laser_transform.translation.y < unit_transform.translation.y - unit.shape.height / 2.
            || laser_transform.translation.y > unit_transform.translation.y + unit.shape.height / 2.
        {
            continue;
        } */

        // if we have colided with the target entity

        // the initial sign is important to detect which way we pass the target, negative or positive

        /* let horizontal_sign = (laser.target_pos.x - laser.start_pos.x).signum();
        let vertical_sign = (laser.target_pos.y - laser.start_pos.y).signum(); */

        // if we have passed or reached the target, despawn the blob
        // && or || might not work as it might move along one axis vert / horiz
        // changed from >= to > to account for above

        /*  if (laser_transform.translation.x - laser.target_pos.x) * horizontal_sign > 0.
            && (laser_transform.translation.y - laser.target_pos.y) * vertical_sign > 0.
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

        let target_angle =
            find_angle(&laser_transform.translation, &unit_transform.translation) + PI;

        /* println!("target angle {}, laser angle {}", target_angle, laser.angle); */

        // clamping is breaking the projectiles

        if laser.angle > target_angle {
            laser.angle = (laser.angle - projectile::TURN_SPEED).clamp(target_angle, f32::MAX);
        } else if laser.angle < target_angle {
            laser.angle = (laser.angle + projectile::TURN_SPEED).clamp(f32::MIN, target_angle);
        }
        // laser.angle = target_angle;

        /* let angle = laser.angle;
        laser_transform.rotation = Quat::from_rotation_z(angle); */

        /* let direction = laser_transform.rotation * Vec3::Y;

        let delta = direction * 200. * time.delta_seconds();
        laser_transform.translation += delta; */

        /*

        let delta_x = 0.;
        let delta_y = 200. * time.delta_seconds() * laser_transform.rotation.y;

        laser_transform.translation.x += delta_x;
        laser_transform.translation.y += delta_y; */

        let angle = laser.angle;
        laser_transform.rotation = Quat::from_rotation_z(angle);

        let direction = laser_transform.rotation * Vec3::Y;

        let speed = Vec3::new(
            (laser.target_pos.x - laser.start_pos.x)
                / SECONDS_PER_TICK
                / PROJECTILE_MOVE_END_TICK_PORTION
                * time.delta_seconds()
                * 2.,
            (laser.target_pos.y - laser.start_pos.y)
                / SECONDS_PER_TICK
                / PROJECTILE_MOVE_END_TICK_PORTION
                * time.delta_seconds()
                * 2.,
            0.,
        );

        let delta = direction * speed.abs();
        laser_transform.translation += delta;

        // use trig to apply evenly for diagonal vs straight movement

        /* let x_delta = (laser.target_pos.x - laser.start_pos.x) / SECONDS_PER_TICK / PROJECTILE_MOVE_END_TICK_PORTION * time.delta_seconds()/* * angle.cos().abs() */;
        let y_delta = (laser.target_pos.y - laser.start_pos.y) / SECONDS_PER_TICK / PROJECTILE_MOVE_END_TICK_PORTION * time.delta_seconds()/* * angle.sin().abs() */;

        laser_transform.translation.x += x_delta;
        laser_transform.translation.y += y_delta; */
    }
}

pub fn create_laser(
    start_pos: &Vec3,
    target_pos: &Vec3,
    target_entity: Entity,
    damage: u32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    /* println!("creating resource blob {:?}", resource); */

    let mut rng = thread_rng();
    let angle_offset = PI * rng.gen_range(projectile::SPAWN_ARC.0..=projectile::SPAWN_ARC.1);

    let angle = find_angle(start_pos, target_pos) + angle_offset + PI;

    /* let mesh = Mesh2dHandle(meshes.add(Circle::new(10.))); */
    let color = laser::COLOR;

    commands.spawn((
        /* MaterialMesh2dBundle {
            mesh,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(start_pos.x, start_pos.y, z_order::PROJECTILE),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        }, */
        SpriteBundle {
            texture: asset_server.load(laser::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(start_pos.x, start_pos.y, 1.0),
                scale: Vec3::new(0.7, 0.7, 1.0),
                rotation: Quat::from_rotation_z(angle),
                ..default()
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
            target_entity,
            angle,
            damage,
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}

pub fn kill_lasers(mut commands: Commands, mut lasers: Query<(&Laser, Entity)>) {
    for (_, entity) in lasers.iter_mut() {
        let comps = entity.components();

        commands.entity(entity).despawn();
    }
}
