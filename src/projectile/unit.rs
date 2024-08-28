use bevy::prelude::*;

use crate::{components::Unit, constants::{PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK}, utils::signed_distance};

pub fn update_units(
    mut units: Query<(&mut Transform, &Unit)>,
    time: Res<Time>,
) {
    
    for (mut transform, unit) in units.iter_mut() {

        let Some(moving) = &unit.moving else {
            continue;
        };

        /* let direction = transform.rotation * Vec3::Y;
        let distance = signed_distance(moving.start_pos, moving.target_pos);

        let translation_delta = distance / SECONDS_PER_TICK * time.delta_seconds() * direction;

        transform.translation += translation_delta; */

        let x_delta = (moving.target_pos.x - moving.start_pos.x) / SECONDS_PER_TICK / PROJECTILE_MOVE_END_TICK_PORTION * time.delta_seconds() /* * direction.x */;
        let y_delta = (moving.target_pos.y - moving.start_pos.y) / SECONDS_PER_TICK / PROJECTILE_MOVE_END_TICK_PORTION * time.delta_seconds() /* * direction.y */;

        transform.translation.x += x_delta;
        transform.translation.y += y_delta;
    }
}