use std::cell::RefCell;

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
    pub const SPEED: f32 = 200.;
    pub const BOOST_SPEED: f32 = 500.;
    pub const Z_POS: f32 = 1000.;
    pub const MAX_SCALE: f32 = 1.;
    pub const MIN_SCALE: f32 = 0.1;
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
    pub static ref SIMPLEX_GENERATOR: Simplex<2> = Source::simplex(42);
}

pub mod resource_noise_tresholds {
    pub const COAL: (f64, f64) = (0.95, 1.);
    pub const MINERALS: (f64, f64) = (0.83, 0.85);
    pub const SCRAP: (f64, f64) = (0.23, 0.25);
}

pub enum Resource {
    Coal,
    Minerals,
    Scrap,
    Energy,
}

/* thread_local! {
    static SIMPLEX_GENERATOR: RefCell<Simplex<2>> = RefCell::new(Source::simplex(42));
} */

/* pub static ref simplex = Source::simplex(42).fbm(5, 0.013, 2.0, 0.5)                        // apply fractal brownian motion
.blend(                                         // apply blending...
    Source::worley(43).scale([0.05, 0.05]),     // ...with scaled worley noise
    Source::worley(44).scale([0.02, 0.02]))     // ...controlled by other worley noise
.lambda(|f| (f * 2.0).sin() * 0.3 + f * 0.7);   // apply a closure to the noise */
