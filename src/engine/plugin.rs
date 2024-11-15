use std::time::Duration;

use ashscript_types::components::health::Health;
use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::*,
    time::common_conditions::on_timer,
    utils::hashbrown::HashSet,
};
use hexx::Hex;

use crate::{
    components::{
        GameObjectKindComp, LoadChunks, LoadedChunks, ProjectileMoveEndEvent, ProjectileMoveEndTimer, State, TickEvent, UnloadedChunks
    },
    constants::{self, PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK},
    networker::handle_network_events,
};

use super::{
    assembler::generate_assemblers_from_keyframe, distributor::generate_distributors_from_keyframe, factory::generate_factories_from_keyframe, projectile::generate_attack_projectiles_from_keyframe, resources::generate_resources_from_keyframe, terrain::generate_tiles, turret::generate_turrets_from_keyframe, unit::{force_units_move, generate_units_from_factory, generate_units_on_chunkload, kill_units, move_units_from_actions, units_attack_from_actions}
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
                    (force_units_move, reset_projectile_move_end_timer),
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
                    (units_attack_from_actions, (move_units_from_actions), generate_units_from_factory, generate_attack_projectiles_from_keyframe).chain(),
                )
                    .chain()
                    .run_if(on_event::<TickEvent>()),
            ).add_systems(Update, (kill_units).run_if(on_event::<ProjectileMoveEndEvent>()));
    }
}

fn reset_projectile_move_end_timer(mut projectile_timer: ResMut<ProjectileMoveEndTimer>, state: Res<State>) {
    projectile_timer.0.reset();
    Timer::from_seconds(
        state.global.last_tick_duration.as_secs_f32() * PROJECTILE_MOVE_END_TICK_PORTION,
        TimerMode::Once,
    );
}

fn projectile_move_end_event(
    mut event_writer: EventWriter<ProjectileMoveEndEvent>,
    mut projectile_timer: ResMut<ProjectileMoveEndTimer>,
    time: Res<Time>,
) {
    projectile_timer.0.tick(time.delta());

    if projectile_timer.0.just_finished() {
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

    for (chunk_hex, _) in state.map.chunks.iter() {
        if loaded_chunks.0.contains(chunk_hex) {
            continue;
        };

        unloaded_chunks.0.insert(*chunk_hex);
        // pre-emptively insert, as we can predict the chunk will be loaded when we trigger the event
        loaded_chunks.0.insert(*chunk_hex);
    }

    // no chunks to load
    if unloaded_chunks.0.is_empty() {
        return;
    }

    event_writer.send(LoadChunks);
}

/* pub fn kill_0_health(
    units: Query<(GameObjectKindComp, &Transform, &Health, Entity)>,
    mut commands: Commands,
    mut game_object_map: MappedGameObjects,
) {
    for (unit, transform, health, entity) in units.iter() {
        if health.current == 0 {
            game_object_map.remove(
                &HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()),
                GameObjectKind::Unit,
            );

            commands.entity(entity).despawn();
            continue;
        }
    }
} */