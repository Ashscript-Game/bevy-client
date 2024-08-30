use std::f32::consts::PI;

use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::*, render::view::RenderLayers, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, utils::hashbrown::HashMap};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};
use rand::{thread_rng, Rng};

use crate::{components::{OccupiesTile, ResourceBlob}, constants::{self, coal_node, projectile, z_order, Resource, PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK}, utils::{find_angle, find_angle_coords, signed_distance}};

pub fn update_resource_blobs(
    mut resource_blobs: Query<(&mut Transform, &mut ResourceBlob, Entity)>,
    targets: Query<(&OccupiesTile, &Transform), Without<ResourceBlob>>,
    time: Res<Time>,
    mut commands: Commands,
) {

    for (mut blob_transform, mut blob, blob_entity) in resource_blobs.iter_mut() {

        // the initial sign is important to detect which way we pass the target, negative or positive 

/*         let horizontal_sign = (blob.target_pos.x - blob.start_pos.x).signum();
        let vertical_sign = (blob.target_pos.y - blob.start_pos.y).signum();
        
        // if we have passed or reached the target, despawn the blob
        // && or || might not work as it might move along one axis vert / horiz
        // changed from >= to > to account for above
        if (blob_transform.translation.x - blob.target_pos.x) * horizontal_sign > 0.
            || (blob_transform.translation.y - blob.target_pos.y) * vertical_sign > 0.
        {
            /* println!("despawning resource blob {:?}", blob.resource); */
            commands.entity(entity).despawn();
            continue;
        } */

        // translate the position of the blob to move linearly (relative x to y) towards the target
        // this should move the blob at a constant time of SECONDS_PER_TICK, no matter the distance or tick rate, it should reach the destination at the speed of the tick rate

        /*let direction = blob_transform.rotation * Vec3::Y;
         let distance = signed_distance(blob.start_pos, blob.target_pos);

        let translation_delta = distance / SECONDS_PER_TICK * time.delta_seconds() * direction;

        blob_transform.translation += translation_delta; */

        // use trig to apply evenly for diagonal vs straight movement

        /* let x_delta = (blob.target_pos.x - blob.start_pos.x) / SECONDS_PER_TICK / PROJECTILE_MOVE_END_TICK_PORTION * time.delta_seconds() /* * direction.x */;
        let y_delta = (blob.target_pos.y - blob.start_pos.y) / SECONDS_PER_TICK / PROJECTILE_MOVE_END_TICK_PORTION * time.delta_seconds() /* * direction.y */;

        blob_transform.translation.x += x_delta;
        blob_transform.translation.y += y_delta; */

        let Ok((_, unit_transform)) = targets.get(blob.target_entity) else {
            commands.entity(blob_entity).despawn();
            continue;
        };

        let blob_aabb = Aabb2d::new(
            blob_transform.translation.truncate(),
            Vec2::new(5., 5.),
        );

        let unit_aabb = Aabb2d::new(
            unit_transform.translation.truncate(),
            Vec2::new(20., 20.),
        );

        if blob_aabb.intersects(&unit_aabb) {
            println!("intersection");
            commands.entity(blob_entity).despawn();
            continue;
        }

        let target_angle =
            find_angle(&blob_transform.translation, &unit_transform.translation) + PI;

        /* println!("target angle {}, laser angle {}", target_angle, laser.angle); */

        if blob.angle > target_angle {
            blob.angle = (blob.angle - projectile::TURN_SPEED).clamp(target_angle, f32::MAX);
        } else if blob.angle < target_angle {
            blob.angle = (blob.angle + projectile::TURN_SPEED).clamp(f32::MIN, target_angle);
        }

        blob_transform.rotation = Quat::from_rotation_z(target_angle/* angle */);

        let direction = blob_transform.rotation * Vec3::Y;

        let speed = Vec3::new(
            (blob.target_pos.x - blob.start_pos.x)
                / SECONDS_PER_TICK
                / PROJECTILE_MOVE_END_TICK_PORTION
                * time.delta_seconds()
                * 2.,
            (blob.target_pos.y - blob.start_pos.y)
                / SECONDS_PER_TICK
                / PROJECTILE_MOVE_END_TICK_PORTION
                * time.delta_seconds()
                * 2.,
            0.,
        );

        let delta = direction * speed.abs();
        blob_transform.translation += delta;
    }
}

pub fn kill_resource_blobs(mut commands: Commands, mut blobs: Query<(&ResourceBlob, Entity)>) {
    for (_, entity) in blobs.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub fn create_resource_blob(
    start_pos: &Vec3,
    target_pos: &Vec3,
    target_entity: Entity,
    resource: &Resource,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    /* println!("creating resource blob {:?}", resource); */

    let mesh = Mesh2dHandle(meshes.add(Circle::new(10.)));

    let mut rng = thread_rng();
    let angle_offset = PI * rng.gen_range(projectile::SPAWN_ARC.0..=projectile::SPAWN_ARC.1);

    let angle = find_angle_coords(
        start_pos.x,
        start_pos.y,
        target_pos.x,
        target_pos.y,
    ) + PI + angle_offset;
    
    let color_resource_map: HashMap<Resource, Color> = HashMap::from([(Resource::Coal, coal_node::COLOR), (Resource::Minerals, constants::mineral_node::COLOR), (Resource::Metal, constants::metal::COLOR)]);
    let color = *color_resource_map.get(resource).unwrap_or(&Color::WHITE);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(
                    start_pos.x,
                    start_pos.y,
                    z_order::PROJECTILE,
                ),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            ..default()
        },
        OmniLightSource2D {
            intensity: 0.05,
            color: color,
            falloff: Vec3::new(2., 2., 0.005),
            ..Default::default()
        },
        ResourceBlob {
            resource: *resource,
            target_pos: *target_pos,
            target_entity,
            angle,
            start_pos: *start_pos,
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}
