use bevy::prelude::*;

use crate::constants::SECONDS_PER_TICK;

#[derive(Resource)]
pub struct ReplenishDistributors(pub Timer);

pub fn setup_distributor_replenishing(mut commands: Commands) {
    commands.insert_resource(ReplenishDistributors(Timer::from_seconds(SECONDS_PER_TICK, TimerMode::Repeating)));
}