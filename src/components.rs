use bevy::{prelude::*, utils::{HashMap, HashSet}};

use crate::constants::{self, Resource};

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

#[derive(Component)]
pub struct Structure;

#[derive(Component)]
pub struct OccupiesTile;

#[derive(Component)]
pub struct Assembler {
    pub output_resource: constants::Resource,
    pub store: Store,
    pub transferring: Option<Vec2>,
}

#[derive(Component)]
pub struct Distributor {
    pub resource: constants::Resource,
    pub store: Store,
}

#[derive(Component, Default)]
pub struct Store {
    pub resources: HashMap<constants::Resource, u32>,
    /// resources that are allowed to be inserted into the structure. If none, accept all resources
    pub allowed_inputs: Option<HashSet<constants::Resource>>,
    pub capacity: u32,
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ResourceBlob {
    pub target_pos: Vec3,
    pub start_pos: Vec3,
    pub resource: Resource,
}