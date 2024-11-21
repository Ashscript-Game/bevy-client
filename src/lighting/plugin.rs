use bevy::{
    app::{App, Plugin},
    prelude::*,
};
use bevy_magic_light_2d::prelude::*;

use crate::components::State;


pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_lights.after(setup_post_processing_camera))
        .add_systems(Update, update_lights);
    }
}

fn generate_lights(mut commands: Commands) {
    /* let mut occluders = vec![];
    let occluder_entity = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
            LightOccluder2D {
                h_size: Vec2::new(0.0, 0.0),
            },
        ))
        .id();

    occluders.push(occluder_entity);

    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("occluders"))
        .push_children(&occluders); */

    // skylight

    commands.spawn((
        SkylightLight2D {
            //color: Color::rgb_u8(93, 158, 179),
            color: Color::srgb_u8(93, 158, 179),
            intensity: 0.025,
        },
        Name::new("global_skylight"),
    ));

    /* commands.spawn((
        SkylightLight2D {
            color: Color::rgb_u8(93, 158, 179),
            intensity: 1.,
        },
        Name::new("global_skylight"),
    )); */
}

fn update_lights(mut query: Query<&mut SkylightLight2D>, state: Res<State>) {
    let mut skylight = query.single_mut();

    let intensity = match state.global.is_day() {
        true => 0.025,
        false => 0.,
    };

    skylight.intensity = intensity
}