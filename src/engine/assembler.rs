use bevy::prelude::*;

use crate::constants::SECONDS_PER_TICK;

#[derive(Resource)]
pub struct ReplenishAssemblers(pub Timer);

pub fn setup_assembler_replenishing(mut commands: Commands) {
    commands.insert_resource(ReplenishAssemblers(Timer::from_seconds(SECONDS_PER_TICK, TimerMode::Repeating)));
}