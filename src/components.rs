use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    utils::{HashMap, HashSet},
};
use enum_map::EnumMap;
use hexx::{hex, Hex};

use crate::{
    constants::{self, Resource, UnitPart},
    utils,
};

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

#[derive(Default, Debug, Clone)]
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

#[derive(Component, Default, Clone)]
pub struct Unit {
    pub body: UnitBody,
    pub health: u32,
    pub age: u32,
    pub energy: u32,
    pub energy_capacity: u32,
    pub store: Store,
    pub name: String,
    pub weight: u32,
    pub moving: Option<Moving>,
}

pub type UnitBody = EnumMap<UnitPart, u32>;

#[derive(Component)]
pub struct Laser {
    pub target_pos: Vec3,
    pub start_pos: Vec3,
    pub angle: f32,
    /// Used to determine the intensity of the projectile's visuals
    pub damage: u32,
}

#[derive(Component, Default, Clone)]
pub struct Moving {
    pub target_pos: Vec3,
    pub start_pos: Vec3,
    pub angle: f32,
}

#[derive(Component, Default)]
pub struct Turret {
    pub energy: u32,
    pub energy_gen: u32,
    pub damage: u32,
    pub range: u32,
    pub store: Store,
}

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct ProjectileMoveEndEvent;

#[derive(Resource)]
pub struct ProjectileMoveEndTimer(pub Timer);

#[derive(Component)]
/// can use element in queries to get components. query.get(element).unwrap()
pub struct UnitMap(pub HashMap<Hex, Entity>);

// impl UnitMap {
//     pub fn insert(&mut self, hex: Hex, entity: Entity) -> Option<Entity> {
//         self.0.insert(utils::hex::pack(hex), entity)
//     }

//     pub fn get(&self, hex: Hex) -> Option<&Entity> {
//         self.0.get(&utils::hex::pack(hex))
//     }
// }

#[derive(SystemParam)]
pub struct MappedUnits<'w, 's> {
    pub entities: Query<'w, 's, &'static mut UnitMap>,
    pub components: Query<'w, 's, (&'static mut Unit, &'static mut Transform)>,
}

impl<'w, 's> MappedUnits<'w, 's> {
    pub fn remove(&mut self, hex: &Hex) {
        self.entities.single_mut().0.remove(hex);
    }

    pub fn insert(&mut self, hex: &Hex, entity: &Entity) {
        self.entities.single_mut().0.insert(*hex, *entity);
    }

    pub fn entity(&self, hex: &Hex) -> Option<&Entity> {
        self.entities.single().0.get(hex)
    }

    pub fn entity_unchecked(&self, hex: &Hex) -> &Entity {
        self.entity(hex).expect("Entity not found")
    }

    pub fn unit(&self, entity: Entity) -> Option<(&Unit, &Transform)> {
        match self.components.get(entity) {
            Ok(tuple) => Some(tuple),
            Err(_) => None,
        }
    }

    /* pub fn unit_mut(&mut self, entity: Entity) -> Option<(&Mut<'w, Unit>, &Mut<'w, Transform>/* &Mut<'w, Unit>, &Mut<'w, Transform> */)> {
        match self.components.get_mut(entity) {
            Ok((unit, transform)) => Some((& unit, &transform)),
            Err(_) => None,
        }
    } */

    pub fn unit_unchecked(&self, entity: Entity) -> (&Unit, &Transform) {
        self.unit(entity).expect("Unit not found")
    }
}

#[derive(SystemParam)]
pub struct MappedTerrain;

#[derive(SystemParam)]
pub struct MappedStructures;

#[derive(Resource)]
pub struct GameSettings {
    pub lights: bool,
}

#[derive(Component)]
pub enum Structure {
    Assembler,
    Distributor,
    Turret,
}