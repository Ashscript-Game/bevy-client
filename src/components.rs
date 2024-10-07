use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    utils::{hashbrown::HashSet, HashMap},
};
use enum_map::EnumMap;
use hexx::Hex;

use crate::constants::{self, Resource, UnitPart};

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
    pub target_entity: Entity,
    pub resource: Resource,
    pub angle: f32,
}

#[derive(Clone)]
pub struct Owner {
    pub name: String,
    pub id: u32,
    // obviously need an ID system at some point
}

#[derive(Component, Default, Clone)]
pub struct Unit {
    pub owner_id: u32,
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
    pub target_entity: Entity,
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
    /* pub components: Query<'w, 's, (&'static mut Unit, &'static mut Transform)>, */
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

    /* pub fn unit(&self, entity: Entity) -> Option<(&Unit, &Transform)> {
        match self.components.get(entity) {
            Ok(tuple) => Some(tuple),
            Err(_) => None,
        }
    } */

    /* pub fn unit_mut(&mut self, entity: Entity) -> Option<(&Mut<'w, Unit>, &Mut<'w, Transform>/* &Mut<'w, Unit>, &Mut<'w, Transform> */)> {
        match self.components.get_mut(entity) {
            Ok((unit, transform)) => Some((& unit, &transform)),
            Err(_) => None,
        }
    } */

    /* pub fn unit_unchecked(&self, entity: Entity) -> (&Unit, &Transform) {
        self.unit(entity).expect("Unit not found")
    } */
}

#[derive(SystemParam)]
pub struct MappedTerrain;

#[derive(Component)]
pub struct OccupyStructuresMap(pub HashMap<Hex, HashSet<Entity>>);

#[derive(SystemParam)]
pub struct MappedOccupyStructures<'w, 's>(pub Query<'w, 's, &'static mut OccupyStructuresMap>);

impl <'w, 's> MappedOccupyStructures<'w, 's> {
    pub fn insert(&mut self, hex: &Hex, entity: &Entity) {
        self.0.single_mut().0.get_mut(hex).unwrap().insert(*entity);
    }

    pub fn remove(&mut self, hex: &Hex, entity: &Entity) {
        self.0.single_mut().0.get_mut(hex).unwrap().remove(entity);
    }

    pub fn get(&self, hex: &Hex) -> Option<&HashSet<Entity>> {
        self.0.single().0.get(hex)
    }

    pub fn get_unchecked(&self, hex: &Hex) -> &HashSet<Entity> {
        self.get(hex).unwrap()
    }
}

#[derive(Resource)]
pub struct GameSettings {
    pub lights: bool,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub units: Vec<(Unit, Transform, Entity)>,
    pub factories: Vec<(Factory, Transform, Entity)>,
    pub players: Vec<Owner>,
    pub walls: HashSet<Hex>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: vec![
                Owner {
                    name: "Player".to_string(),
                    id: 0,
                },
                Owner {
                    name: "AI".to_string(),
                    id: 1,
                },
            ],
            ..default()
        }
    }
}

pub mod intents {
    use bevy::prelude::*;
    use hexx::Hex;

    use crate::constants::Resource;

    pub struct UnitAttack {
        pub attacker: Entity,
        pub target: Entity,
    }

    pub struct UnitMove {
        pub entity: Entity,
        pub to: Hex,
    }

    pub struct ResourceTransfer {
        pub resource: Resource,
        pub amount: u32,
        pub from: Entity,
        pub to: Entity,
    }

    pub struct FactorySpawn {
        pub entity: Entity,
        pub out: Option<Hex>,
    }
}

#[derive(Resource, Default)]
/// should probably spacialize this at some point, to align with chunks
pub struct Intents {
    pub unit_attack: Vec<intents::UnitAttack>,
    pub unit_move: Vec<intents::UnitMove>,
    pub resource_transfer: Vec<intents::ResourceTransfer>,
    pub factory_spawn: Vec<intents::FactorySpawn>,
}

impl Intents {
    pub fn new() -> Self {
        Self {
            ..default()
        }
    }
}

pub struct PlayerState {
    pub memory: HashMap<String, String>,
    pub intents: Intents,
    pub owner_id: u32,
}

impl PlayerState {
    pub fn new(owner_id: u32) -> Self {
        Self {
            memory: HashMap::new(),
            intents: Intents::new(),
            owner_id,
        }
    }
}

#[derive(Resource)]
pub struct PlayerStates(pub HashMap<String, PlayerState>);

// #[derive(Resource)]
// pub struct PlayerScripts<'a>(pub Vec<&'a dyn Fn(Res<GameState>, Res<PlayerState>)>);

// impl PlayerScripts<'_> {
//     pub fn new() -> Self {
//         Self(Vec::new())
//     }
// }


#[derive(Component)]
pub enum Structure {
    Assembler,
    Distributor,
    Turret,
}

#[derive(Component, Default, Clone)]
pub struct Factory {
    pub store: Store,
    pub owner_id: u32,
    pub energy: u32,
    pub energy_capacity: u32,
    // 100(%)+ = completed
    pub production_progress: u8,
}

#[derive(Component)]
pub struct Wall;
