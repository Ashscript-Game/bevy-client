use ashscript_types::{components::{body::UnitBody, energy::Energy, health::Health, resource::ResourceNode, solar_panel::SolarPanel, storage::Storage, substation::Substation, turbine::Turbine}, global::Global, keyframe::KeyFrame, map::Map, objects::GameObjectKind};
use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    utils::{hashbrown::HashSet, HashMap},
};
use enum_map::EnumMap;
use hexx::Hex;
use uuid::Uuid;
use hecs;

use crate::constants::{self, Resource, UnitPart};

#[derive(Component)]
pub struct ResourceNodeComp(pub ResourceNode);

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

#[derive(Component)]
pub struct Owner(pub Uuid);

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub id: Uuid,
}

#[derive(Component)]
pub struct GameObjectKindComp(pub GameObjectKind);

#[derive(Component)]
pub struct HealthComp(pub Health);

#[derive(Component)]
pub struct EnergyComp(pub Energy);

#[derive(Component)]
pub struct StorageComp(pub Storage);

#[derive(Component, Default, Clone)]
pub struct Unit {
    pub store: Store,
    pub name: String,
    pub weight: u32,
    pub moving: Option<Moving>,
}

#[derive(Component)]
pub struct UnitBodyComp(pub UnitBody);

#[derive(Component)]
pub struct SubstationComp(pub Substation);

#[derive(Component)]
pub struct TurbineComp(pub Turbine);

#[derive(Component)]
pub struct SolarPanelComp(pub SolarPanel);

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
    pub store: Store,
}

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct LoadChunks;

#[derive(Event)]
pub struct ProjectileMoveEndEvent;

#[derive(Resource)]
pub struct ProjectileMoveEndTimer(pub Timer);

#[derive(Component)]
/// can use element in queries to get components. query.get(element).unwrap()
pub struct GameObjectMap(pub EnumMap<GameObjectKind, HashMap<Hex, Entity>>);

#[derive(Component)]
pub struct ServerClientEntitiesMap(pub HashMap<hecs::Entity, Entity>);

#[derive(SystemParam)]
pub struct MappedGameObjects<'w, 's> {
    pub entities: Query<'w, 's, &'static mut GameObjectMap>,
    pub server_client_entities: Query<'w, 's, &'static ServerClientEntitiesMap>,
}

impl<'w, 's> MappedGameObjects<'w, 's> {
    pub fn remove(&mut self, hex: &Hex, kind: GameObjectKind) -> Option<Entity> {
        self.entities.single_mut().0[kind].remove(hex)
    }

    pub fn insert(&mut self, hex: Hex, kind: GameObjectKind, entity: Entity) {
        self.entities.single_mut().0[kind].insert(hex, entity);
    }

    pub fn entity(&self, hex: &Hex, kind: GameObjectKind) -> Option<&Entity> {
        self.entities.single().0[kind].get(hex)
    }

    pub fn entity_unchecked(&self, hex: &Hex, kind: GameObjectKind) -> &Entity {
        self.entity(hex, kind).expect("Entity not found")
    }

    pub fn move_to(&mut self, from: &Hex, to: Hex, kind: GameObjectKind) -> Option<()> {
        let entity = self.remove(from, kind)?;
        self.insert(to, kind, entity);

        Some(())
    }

    pub fn entity_from_server(&self, entity: hecs::Entity) -> Option<&Entity> {
        self.server_client_entities.single().0.get(&entity)
    }

    pub fn entity_from_server_unchecked(&self, entity: hecs::Entity) -> &Entity {
        self.entity_from_server(entity).expect("Entity not found")
    }
}

#[derive(Component)]
pub struct ServerEntity(pub hecs::Entity);

#[derive(Resource)]
pub struct GameSettings {
    pub lights: bool,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub units: Vec<(Unit, Transform, Entity)>,
    pub factories: Vec<(Factory, Transform, Entity)>,
    pub players: Vec<Player>,
    pub walls: HashSet<Hex>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: vec![
                Player {
                    name: "Player".to_string(),
                    id: Uuid::default(),
                },
                Player {
                    name: "AI".to_string(),
                    id: Uuid::default(),
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
    // 100(%)+ = completed
    pub production_progress: u8,
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Lava;

#[derive(Resource)]
pub struct State {
    pub map: Map,
    pub global: Global,
    pub world: hecs::World,
}

#[derive(Resource)]
pub struct Actions(pub ashscript_types::actions::ActionsByKind);

#[derive(Resource, Default)]
pub struct LoadedChunks(pub HashSet<Hex>);

#[derive(Resource, Default)]
pub struct UnloadedChunks(pub HashSet<Hex>);

#[derive(Component)]
pub struct Selected;

#[derive(Resource, Default)]
pub struct SelectedGameObjects(pub HashSet<Entity>);

#[derive(Resource, Default)]
pub struct DebugUI {
    pub enabled: bool,
    pub chunk_lines: bool,
}

#[derive(Component)]
pub struct ScrollableCamera;

#[derive(Component)]
pub struct MinimapCamera;