use std::time::Duration;

use bevy::{
    app::{App, Plugin, Update},
    prelude::{on_event, IntoSystemConfigs},
    time::common_conditions::on_timer,
};

use crate::{components::ProjectileMoveEndEvent, constants};

use super::{
    assembler::{assembler_ai, assemblers_produce},
    distributor::distributor_ai,
    unit::{units_attack, units_move, units_stop_move},
};

pub struct PlayerScriptPlugin;

impl Plugin for PlayerScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                distributor_ai,
                (assemblers_produce, assembler_ai).chain(),
                (units_move, units_attack).chain(),
            )
                .run_if(on_timer(Duration::from_secs_f32(
                    constants::SECONDS_PER_TICK,
                ))),
        )
        .add_systems(
            Update,
            units_stop_move.run_if(on_event::<ProjectileMoveEndEvent>()),
        );
    }
}
