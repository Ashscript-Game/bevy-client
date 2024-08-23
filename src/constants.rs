use std::{cell::RefCell, collections::HashSet};

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
    pub const COLOR: Color = Color::Rgba {
        red: 240. / 255.,
        green: 240. / 255.,
        blue: 60. / 255.,
        alpha: 1.,
    };
}

pub mod coal_node {
    use bevy::prelude::*;

    pub const COLOR: Color = Color::Rgba {
        red: 20. / 255.,
        green: 20. / 255.,
        blue: 20. / 255.,
        alpha: 1.,
    };
}

pub mod mineral_node {
    use bevy::prelude::*;

    pub const COLOR: Color = Color::Rgba {
        red: 120. / 255.,
        green: 240. / 255.,
        blue: 120. / 255.,
        alpha: 1.,
    };
}

pub mod scrap {
    use bevy::prelude::*;

    pub const ASSET_PATH: &str = "scout_factory.png";
    pub const LIFETIME_PER_METAL: u32 = 3;
    pub const LIFETIME_OFFSET: u32 = 50;
    pub const Z_POS: f32 = 1.;
    pub const COLOR: Color = Color::Rgba {
        red: 100. / 255.,
        green: 100. / 255.,
        blue: 100. / 255.,
        alpha: 1.,
    };
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
}

pub mod resource_noise_tresholds {
    pub const WALL: (f64, f64) = (0.15, 1.);
    pub const COAL: (f64, f64) = (-0.18, -0.18);
    pub const MINERALS: (f64, f64) = (0.148, 0.15);
    pub const SCRAP: (f64, f64) = (-0.23, -0.25);
}

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy)]
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
    pub const RANGE: u32 = 2;
}

pub mod distributor {
    use bevy::prelude::*;

    pub const ASSET_PATH: &str = "dist.png";
    pub const STORE_CAPACITY: u32 = 1000;
    pub const COLOR: Color = Color::Rgba {
        red: 241. / 255.,
        green: 240. / 255.,
        blue: 110. / 255.,
        alpha: 1.,
    };
    pub const RANGE: u32 = 4;
}

pub mod wall {
    use bevy::prelude::*;

    pub const COLOR: Color = Color::Rgba {
        red: 0. / 255.,
        green: 0. / 255.,
        blue: 0. / 255.,
        alpha: 1.,
    };
}

pub mod z_order {
    pub const PROJECTILE: f32 = 100.;
}

pub mod resource_blob {
}

pub const SECONDS_PER_TICK: f32 = 3.;

/* thread_local! {
    static SIMPLEX_GENERATOR: RefCell<Simplex<2>> = RefCell::new(Source::simplex(42));
} */

/* pub static ref simplex = Source::simplex(42).fbm(5, 0.013, 2.0, 0.5)                        // apply fractal brownian motion
.blend(                                         // apply blending...
    Source::worley(43).scale([0.05, 0.05]),     // ...with scaled worley noise
    Source::worley(44).scale([0.02, 0.02]))     // ...controlled by other worley noise
.lambda(|f| (f * 2.0).sin() * 0.3 + f * 0.7);   // apply a closure to the noise */
