use std::time::Duration;

use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::{
    components::{ProjectileMoveEndEvent, ProjectileMoveEndTimer, TickEvent},
    constants::{self, PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK},
};

use super::{terrain::generate_tiles, unit::kill_units};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>()
            .add_event::<ProjectileMoveEndEvent>()
            .add_systems(Update, (projectile_move_end_event,))
            .observe(generate_tiles);
    }
}

fn projectile_move_end_event(
    mut event_writer: EventWriter<ProjectileMoveEndEvent>,
    mut projectile_timer: ResMut<ProjectileMoveEndTimer>,
    time: Res<Time>,
) {
    projectile_timer.0.tick(time.delta());

    if projectile_timer.0.finished() {
        event_writer.send(ProjectileMoveEndEvent);
    }
}

/*fn on_tick(mut event_reader: EventReader<TickEvent>) {
    for (event, _) in event_reader.read_with_id() {
        // event.projectile_move_end_event.tick(Duration::from_secs_f32(SECONDS_PER_TICK));

        println!("tick event happened");
    }
}*/
