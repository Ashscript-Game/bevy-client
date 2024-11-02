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

use super::{
    benchmarks::factory_combat_benchmark,
    factory::progress_factories,
    player::{
        run_factory_spawn_intents, run_move_intents,
        run_unit_attack_intents,
    },
    terrain::generate_tiles,
    unit::{kill_units},
};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                generate_tiles,
                /* assembler_distributor_benchmark, */
                /* unit_benchmark, */
                factory_combat_benchmark,
                /* turret_benchmark, */
            )
                .chain(),
        )
        .add_event::<TickEvent>()
        .add_event::<ProjectileMoveEndEvent>()
        .add_systems(
            Update,
            (
                projectile_move_end_event,
                (
                    tick_event,
                    /* units_stop_move, */
                    (kill_units, progress_factories),
                    (
                        run_move_intents,
                        run_factory_spawn_intents,
                        run_unit_attack_intents,
                    ),
                )
                    .run_if(on_timer(Duration::from_secs_f32(
                        constants::SECONDS_PER_TICK,
                    ))),
            ),
        );
    }
}

fn tick_event(
    mut event_writer: EventWriter<TickEvent>,
    mut projectile_timer: ResMut<ProjectileMoveEndTimer>,
) {
    event_writer.send(TickEvent);

    projectile_timer.0 = Timer::from_seconds(
        SECONDS_PER_TICK * PROJECTILE_MOVE_END_TICK_PORTION,
        TimerMode::Once,
    );
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