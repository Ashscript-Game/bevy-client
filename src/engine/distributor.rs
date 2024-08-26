use bevy::prelude::*;

use crate::constants::SECONDS_PER_TICK;

#[derive(Resource)]
pub struct Tick(pub Timer);

pub fn setup_distributor_replenishing(mut commands: Commands) {
    commands.insert_resource(Tick(Timer::from_seconds(SECONDS_PER_TICK, TimerMode::Repeating)));
}