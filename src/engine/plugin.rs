use std::time::Duration;

use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::*,
    time::common_conditions::on_timer,
    utils::hashbrown::HashSet,
};
use hexx::Hex;

use crate::{
    components::{
        LoadChunks, LoadedChunks, ProjectileMoveEndEvent, ProjectileMoveEndTimer, State, TickEvent,
        UnloadedChunks,
    },
    constants::{self, PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK},
    networker::handle_network_events,
};

use super::{
    assembler::generate_assemblers_from_keyframe,
    distributor::generate_distributors_from_keyframe,
    factory::generate_factories_from_keyframe,
    resources::generate_resources_from_keyframe,
    terrain::generate_tiles,
    turret::generate_turrets_from_keyframe,
    unit::{units_attack_from_actions, force_units_move, generate_units_from_factory, generate_units_on_chunkload, move_units_from_actions},
    projectile::generate_attack_projectiles_from_keyframe,
};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>()
            .add_event::<ProjectileMoveEndEvent>()
            .add_event::<LoadChunks>()
            .add_systems(Update, (projectile_move_end_event,))
            .add_systems(
                Update,
                (
                    (force_units_move,),
                    chunk_load_update_events,
                    (
                        generate_tiles,
                        generate_units_on_chunkload,
                        generate_resources_from_keyframe,
                        generate_factories_from_keyframe,
                        generate_assemblers_from_keyframe,
                        generate_turrets_from_keyframe,
                        generate_distributors_from_keyframe,
                    )
                        .run_if(on_event::<LoadChunks>()),
                    ((move_units_from_actions), units_attack_from_actions, generate_units_from_factory, generate_attack_projectiles_from_keyframe).chain(),
                )
                    .chain()
                    .run_if(on_event::<TickEvent>()),
            );
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

pub fn chunk_load_update_events(
    state: Res<State>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    mut unloaded_chunks: ResMut<UnloadedChunks>,
    mut event_writer: EventWriter<LoadChunks>,
) {
    println!("finding chunks to load");

    unloaded_chunks.0.clear();

    for (hex, _) in state.map.chunks.iter() {
        if loaded_chunks.0.contains(hex) {
            continue;
        };

        unloaded_chunks.0.insert(*hex);
        // pre-emptively insert, as we can predict the chunk will be loaded when we trigger the event
        loaded_chunks.0.insert(*hex);
    }

    // no chunks to load
    if unloaded_chunks.0.is_empty() {
        return;
    }

    event_writer.send(LoadChunks);
}