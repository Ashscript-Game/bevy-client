use std::time::Duration;

use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::{constants, structure, unit};

use super::{
    benchmarks::{assembler_distributor_benchmark, unit_benchmark},
    resources::generate_resources,
    terrain::generate_tiles,
    unit::{age_units, energize_units, kill_units},
};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                generate_tiles,
                generate_resources,
                /* assembler_distributor_benchmark, */
                unit_benchmark,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (age_units, kill_units, energize_units).run_if(on_timer(Duration::from_secs_f32(
                constants::SECONDS_PER_TICK,
            ))),
        );
    }
}
