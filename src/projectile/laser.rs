use core::f32;
use std::f32::consts::PI;

use bevy::{
    ecs::observer::TriggerTargets,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};
use rand::{thread_rng, Rng};

use crate::{
    components::{Laser, Unit},
    constants::{
        laser, projectile, PROJECTILE_MOVE_END_TICK_PORTION,
        SECONDS_PER_TICK,
    },
    utils::find_angle,
};

pub fn update_lasers(
    mut lasers: Query<(&mut Transform, &mut Laser, Entity), Without<Unit>>,
    units: Query<(&Unit, &Transform), Without<Laser>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut laser_transform, mut laser, laser_entity) in lasers.iter_mut() {
        let Ok((_, unit_transform)) = units.get(laser.target_entity) else {
            commands.entity(laser_entity).despawn();
            continue;
        };

        let laser_aabb = Aabb2d::new(laser_transform.translation.truncate(), Vec2::new(5., 5.));

        let unit_aabb = Aabb2d::new(unit_transform.translation.truncate(), Vec2::new(20., 20.));

        if laser_aabb.intersects(&unit_aabb) {
            
            commands.entity(laser_entity).despawn();
            continue;
        }

        let target_angle =
            find_angle(&laser_transform.translation, &unit_transform.translation) + PI;

        /* println!("target angle {}, laser angle {}", target_angle, laser.angle); */

        if laser.angle > target_angle {
            laser.angle = (laser.angle - projectile::TURN_SPEED).clamp(target_angle, f32::MAX);
        } else if laser.angle < target_angle {
            laser.angle = (laser.angle + projectile::TURN_SPEED).clamp(f32::MIN, target_angle);
        }

        laser_transform.rotation = Quat::from_rotation_z(target_angle/* laser.angle */);

        let direction = laser_transform.rotation * Vec3::new(0., 1., 0.);

        let delta_seconds = time.delta_seconds();
        let speed = Vec3::new(
            (laser.target_pos.x/* unit_transform.translation.x */ - laser.start_pos.x)
                / SECONDS_PER_TICK
                / PROJECTILE_MOVE_END_TICK_PORTION
                * delta_seconds
                * 2.
                + projectile::DEFAULT_SPEED * delta_seconds,
            (laser.target_pos.y/* unit_transform.translation.y */ - laser.start_pos.y)
                / SECONDS_PER_TICK
                / PROJECTILE_MOVE_END_TICK_PORTION
                * delta_seconds
                * 2.
                + projectile::DEFAULT_SPEED * delta_seconds,
            0.,
        );

        let delta = direction * speed.abs();
        laser_transform.translation += delta;
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
                rotation: Quat::from_rotation_z(angle)
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
        let _comps = entity.components();

        commands.entity(entity).despawn();
    }
}
