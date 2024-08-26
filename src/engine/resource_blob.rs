use bevy::prelude::*;

use crate::{components::ResourceBlob, constants::SECONDS_PER_TICK, utils::signed_distance};

pub fn update_resource_blobs(
    mut resource_blobs: Query<(&mut Transform, &ResourceBlob, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {

    for (mut blob_transform, blob, entity) in resource_blobs.iter_mut() {

        // the initial sign is important to detect which way we pass the target, negative or positive 

        let horizontal_sign = (blob.target_pos.x - blob.start_pos.x).signum();
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
        }

        // translate the position of the blob to move linearly (relative x to y) towards the target
        // this should move the blob at a constant time of SECONDS_PER_TICK, no matter the distance or tick rate, it should reach the destination at the speed of the tick rate

        let distance = signed_distance(blob.start_pos, blob.target_pos);

        let direction = blob_transform.rotation * Vec3::Y;
        let translation_delta = distance / SECONDS_PER_TICK * time.delta_seconds() * direction;

        blob_transform.translation += translation_delta;
    }
}