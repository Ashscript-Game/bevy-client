use std::time::Duration;

use bevy::{app::{App, Plugin, Update}, prelude::*, time::common_conditions::on_timer};

use crate::constants;

use super::{laser::{kill_lasers, update_lasers}, resource_blob::{kill_resource_blobs, update_resource_blobs}, unit::{update_units}};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_resource_blobs, update_lasers, update_units, (kill_lasers, kill_resource_blobs).run_if(on_timer(Duration::from_secs_f32(
            constants::SECONDS_PER_TICK,
        )))));
    }
}