use std::time::Duration;

use bevy::{
    app::{App, Plugin, Update},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::{components::ProjectileMoveEndEvent, constants, player_script::unit::units_stop_move};

use super::{
    laser::{kill_lasers, update_lasers},
    resource_blob::{kill_resource_blobs, update_resource_blobs},
    unit::update_units,
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
