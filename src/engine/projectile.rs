use crate::{components::{Actions, Health, MappedGameObjects}, projectile::laser::create_laser};
use ashscript_types::constants::map::HEX_LAYOUT;
use bevy::prelude::*;

pub fn generate_attack_projectiles_from_keyframe(
    actions: Res<Actions>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    targets: Query<(&Transform, &Health)>,
    game_object_map: MappedGameObjects,
) {
    for action in actions.0.unit_attack.iter() {
        let Some(target_entity) = game_object_map.entity(&action.target_hex, action.target_kind)  else {
            continue;
        };

        let start_pos = HEX_LAYOUT.hex_to_world_pos(action.attacker_hex).extend(0.);
        let target_pos = HEX_LAYOUT.hex_to_world_pos(action.target_hex).extend(0.);
        create_laser(&start_pos, &target_pos, *target_entity, 1, commands, asset_server);
    }
}