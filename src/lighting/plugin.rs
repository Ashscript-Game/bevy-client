use bevy::{
    app::{App, Plugin, Update},
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
};
use bevy_magic_light_2d::{prelude::*, FloorCamera, SpriteCamera};
use hexx::hex;

use crate::terrain::tiles::HEX_LAYOUT;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_lights.after(setup_post_processing_camera));
    }
}

fn generate_lights(mut commands: Commands, camera_targets: Res<CameraTargets>) {
    let mut occluders = vec![];
    let occluder_entity = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
            LightOccluder2D {
                h_size: Vec2::new(40.0, 20.0),
            },
        ))
        .id();

    occluders.push(occluder_entity);

    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("occluders"))
        .push_children(&occluders);

    // skylight

    commands.spawn((
        SkylightLight2D {
            color: Color::rgb_u8(93, 158, 179),
            intensity: 0.025,
        },
        Name::new("global_skylight"),
    ));
}
