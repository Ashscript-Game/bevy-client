use std::time::Duration;

use bevy::{
    app::{App, Plugin, Update},
    prelude::IntoSystemConfigs,
    time::common_conditions::on_timer,
};

use crate::constants;

use super::{
    assembler::{assembler_ai, assemblers_produce},
    distributor::distributor_ai,
};

pub struct PlayerScriptPlugin;

impl Plugin for PlayerScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (distributor_ai, (assemblers_produce, assembler_ai).chain()).run_if(on_timer(
                Duration::from_secs_f32(constants::SECONDS_PER_TICK),
            )),
        );
    }
}
