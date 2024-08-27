use bevy::app::{App, Plugin, Update};

use super::{laser::update_lasers, resource_blob::update_resource_blobs};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_resource_blobs, update_lasers));
    }
}