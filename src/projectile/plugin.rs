use bevy::{
    app::{App, Plugin, Update},
    prelude::*,
};

use crate::components::ProjectileMoveEndEvent;

use super::{
    laser::update_lasers,
    resource_blob::update_resource_blobs,
    unit::{units_stop_move, update_units},
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_lasers,
                update_units,
                update_resource_blobs,
            ),
        )
        .add_systems(
            Update,
            units_stop_move.run_if(on_event::<ProjectileMoveEndEvent>()),
        )
        /* .add_systems(
            Update,
            (kill_lasers, kill_resource_blobs).run_if(on_event::<ProjectileMoveEndEvent>()),
        ) */;
    }
}
