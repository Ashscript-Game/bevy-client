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
    pub const SPEED: f32 = 150.;
    pub const BOOST_SPEED: f32 = 220.;
    pub const Z_POS: f32 = 1000.;
}

pub enum ResultCode {
    Success,
    Failure,
    NoAction,
}