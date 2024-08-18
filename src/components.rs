use bevy::prelude::*;

#[derive(Component)]
pub struct ResourceNode {
    pub coal_percent: u32,
    pub mineral_percent: u32,
    pub ticks_to_regen: u32,
    pub resource_remaining: u32,
}

#[derive(Component)]
pub struct Scrap {
    pub metal: u32,
    pub ticks_to_decay: u32,
}