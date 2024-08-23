use bevy::app::{App, Plugin, Startup};

use super::{assembler::AssemblerPlugin, distributor::DistributorPlugin};

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AssemblerPlugin, DistributorPlugin));
    }
}