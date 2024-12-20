use ashscript_types::{constants::map::HEX_LAYOUT, objects::GameObjectKind};
use bevy::prelude::*;

use crate::{
    components::{MappedGameObjects, State, Unit},
    constants::{PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK},
};

pub fn update_units(mut units: Query<(&mut Transform, &Unit)>, time: Res<Time>, state: Res<State>) {
    for (mut transform, unit) in units.iter_mut() {
        let Some(moving) = &unit.moving else {
            continue;
        };

        /* let direction = transform.rotation * Vec3::Y;
        let distance = signed_distance(moving.start_pos, moving.target_pos);

        let translation_delta = distance / SECONDS_PER_TICK * time.delta_seconds() * direction;

        transform.translation += translation_delta; */

        let x_delta = (moving.target_pos.x - moving.start_pos.x) / (state.global.last_tick_duration.as_secs_f32() * PROJECTILE_MOVE_END_TICK_PORTION)  * time.delta_seconds() /* * direction.x */;
        let y_delta = (moving.target_pos.y - moving.start_pos.y) / (state.global.last_tick_duration.as_secs_f32() * PROJECTILE_MOVE_END_TICK_PORTION)  * time.delta_seconds() /* * direction.y */;

        transform.translation.x += x_delta;
        transform.translation.y += y_delta;
    }
}

pub fn units_stop_move(
    mut units: Query<(&mut Unit, &mut Transform)>,
    mut game_object_map: MappedGameObjects,
) {
    for (mut unit, mut unit_transform) in units.iter_mut() {
        let Some(moving) = &unit.moving else {
            continue;
        };
        let from_hex = HEX_LAYOUT.world_pos_to_hex(moving.start_pos.truncate());
        let target_hex = HEX_LAYOUT.world_pos_to_hex(moving.target_pos.truncate());

        unit_transform.translation = moving.target_pos;
        unit.moving = None;

        game_object_map.move_to(&from_hex, target_hex, GameObjectKind::Unit);
    }
}
