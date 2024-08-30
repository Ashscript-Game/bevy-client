use std::cell::RefCell;

use bevy::utils::hashbrown::HashSet;
use enum_map::{enum_map, EnumMap};
use lazy_static::lazy_static;
use libnoise::prelude::*;

pub mod control_keys {
    use bevy::input::keyboard::KeyCode;

    pub const MOVE_UP: KeyCode = KeyCode::KeyW;
    pub const MOVE_DOWN: KeyCode = KeyCode::KeyS;
    pub const MOVE_LEFT: KeyCode = KeyCode::KeyA;
    pub const MOVE_RIGHT: KeyCode = KeyCode::KeyD;
    pub const BOOST: KeyCode = KeyCode::ShiftLeft;
    pub const SHOOT: KeyCode = KeyCode::Space;
}

pub mod camera {
    pub const SPEED: f32 = 50.;
    pub const BOOST_SPEED: f32 = 100.;
    pub const BLEND_RATIO: f32 = 0.2;
    pub const Z_POS: f32 = 1000.;
    pub const MAX_SCALE: f32 = 10.;
    pub const MIN_SCALE: f32 = 1.;
}

pub enum ResultCode {
    Success,
    Failure,
    NoAction,
}

pub mod resource_node {
    use bevy::prelude::*;

    pub const ASSET_PATH: &str = "grass.png";
    pub const Z_POS: f32 = 1.;
    pub const COLOR: Color = Color::srgba(240. / 255., 240. / 255., 60. / 255., 1.);
}

pub mod coal_node {
    use bevy::prelude::*;

    pub const COLOR: Color = Color::srgba(20. / 255., 20. / 255., 20. / 255., 1.);
}

pub mod mineral_node {
    use bevy::prelude::*;

    pub const COLOR: Color = Color::srgba(120. / 255., 240. / 255., 120. / 255., 1.);
}

pub mod scrap {
    use bevy::prelude::*;

    pub const ASSET_PATH: &str = "scout_factory.png";
    pub const LIFETIME_PER_METAL: u32 = 3;
    pub const LIFETIME_OFFSET: u32 = 50;
    pub const Z_POS: f32 = 1.;
    pub const COLOR: Color = Color::srgba(100. / 255., 100. / 255., 100. / 255., 1.);
}

/* pub const VAR: Simplex<2> = Source::simplex(42); */

lazy_static! {
    /* pub static ref SIMPLEX_GENERATOR: libnoise::Scale<2, Simplex<2>> = Source::simplex(42).scale([100., 100.]); */
    pub static ref SIMPLEX_GENERATOR: Blend<2, Fbm<2, Simplex<2>>, Scale<2, Worley<2>>, Scale<2, Worley<2>>> = Source::simplex(43)                 // start with simplex noise
    .fbm(5, 0.013, 2.0, 0.5)                        // apply fractal brownian motion
    .blend(                                         // apply blending...
        Source::worley(43).scale([0.05, 0.05]),     // ...with scaled worley noise
        Source::worley(44).scale([0.02, 0.02]));     // ...controlled by other worley noise
       // apply a closure to the noise
    pub static ref RESOURCE_INPUTS: EnumMap<Resource, HashSet<Resource>> = enum_map! {
        Resource::Metal => HashSet::from([Resource::Coal, Resource::Minerals]),
        Resource::Energy => HashSet::new(),
        Resource::Coal => HashSet::new(),
        Resource::Scrap => HashSet::new(),
        Resource::Minerals => HashSet::new(),
    };
    pub static ref UNIT_PART_WEIGHTS: EnumMap<UnitPart, u32> = enum_map! {
        UnitPart::Ranged => 5,
        UnitPart::Generate => 2,
        UnitPart::Battery => 4,
        UnitPart::Harvest => 2,
        _ => 1,
    };
}

pub mod resource_noise_tresholds {
    pub const WALL: (f64, f64) = (0.15, 1.);
    pub const COAL: (f64, f64) = (-0.18, -0.18);
    pub const MINERALS: (f64, f64) = (0.148, 0.15);
    pub const SCRAP: (f64, f64) = (-0.23, -0.25);
}

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Resource {
    Coal,
    Minerals,
    Scrap,
    Energy,
    Metal,
}

pub mod assembler {
    pub const ASSET_PATH: &str = "assembler.png";
    pub const STORE_CAPACITY: u32 = 1000;
    pub const RANGE: u32 = 3;
}

pub mod distributor {
    use bevy::prelude::*;

    pub const ASSET_PATH: &str = "dist.png";
    pub const STORE_CAPACITY: u32 = 1000;
    pub const COLOR: Color = Color::srgba(241. / 255., 240. / 255., 110. / 255., 1.);
    pub const RANGE: u32 = 3;
}

pub mod wall {
    use bevy::prelude::*;

    pub const COLOR: Color = Color::srgba(0. / 255., 0. / 255., 0. / 255., 1.);
}

pub mod metal {
    use bevy::prelude::*;

    pub const COLOR: Color = Color::srgba(200. / 255., 200. / 255., 200. / 255., 1.);
}

pub mod z_order {
    pub const PROJECTILE: f32 = 100.;
}

pub mod resource_blob {}

pub const SECONDS_PER_TICK: f32 = 2.;
pub const PROJECTILE_MOVE_END_TICK_PORTION: f32 = 0.75;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum GeneralResult {
    Success,
    Fail,
}

pub mod unit {
    use bevy::prelude::*;

    pub const ASSET_PATH: &str = "player.png";
    pub const MAX_PARTS: u32 = 100;
    pub const MAX_HEALTH: u32 = 100;
    pub const MAX_AGE: u32 = 100;
    pub const COLOR: Color = /* Color::Srgba {
            red: 150. / 255.,
            green: 150. / 255.,
            blue: 150. / 255.,
            alpha: 1.,
        }; */
        Color::srgba(150. / 255., 150. / 255., 150. / 255., 1.);
    pub const LIGHT_COLOR: Color = Color::srgba(241. / 255., 240. / 255., 110. / 255., 1.);
}

#[derive(enum_map::Enum)]
pub enum UnitPart {
    Ranged,
    Harvest,
    Generate,
    Work,
    Battery,
}

pub mod laser {
    use bevy::prelude::*;

    pub const COLOR: Color = Color::srgba(240. / 255., 0. / 255., 0. / 255., 1.);
    pub const ASSET_PATH: &str = "laser.png";
}

pub mod turret {
    use bevy::color::Color;

    pub const ASSET_PATH: &str = "turret.png";
    pub const STORE_CAPACITY: u32 = 1000;
    pub const COLOR: Color = Color::srgba(241. / 255., 240. / 255., 110. / 255., 1.);
}

pub mod map {
    /// allows for a rectangle with 2 025 000 000 tiles
    pub const MAX_WIDTH_HEIGHT: i32 = 45000;
}

pub mod projectile {
    /// Turn speed in radians
    pub const TURN_SPEED: f32 = 0.1;
}