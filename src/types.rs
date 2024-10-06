use bevy::prelude::*;

use crate::components::{GameState, PlayerState};

pub type PlayerScript = fn(&Res<GameState>, &mut PlayerState);