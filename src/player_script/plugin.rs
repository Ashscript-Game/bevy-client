
use bevy::{
    app::{App, Plugin, Update},
    prelude::{on_event, IntoSystemConfigs},
};

use crate::components::TickEvent;

use super::{
    assembler::{assembler_ai, assemblers_produce}, distributor::distributor_ai, turret::turret_ai, unit::{units_attack, units_move}
};

pub struct PlayerScriptPlugin;

impl Plugin for PlayerScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                distributor_ai,
                (assemblers_produce, assembler_ai).chain(),
                (units_move, units_attack, turret_ai).chain(),
            )
                .run_if(on_event::<TickEvent>()),
        );
        /*         .add_systems(
            Update,
            units_stop_move.run_if(on_event::<ProjectileMoveEndEvent>()),
        ) */
    }
}
