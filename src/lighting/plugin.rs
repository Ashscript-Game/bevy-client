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

    // Add lights.
    let mut lights = vec![];
    {
        let spawn_light = |cmd: &mut Commands,
                           x: f32,
                           y: f32,
                           name: &'static str,
                           light_source: OmniLightSource2D| {
            return cmd
                .spawn(Name::new(name))
                .insert(light_source)
                .insert(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(RenderLayers::all())
                /* .insert(RenderLayers::from_layers(CAMERA_LAYER_OBJECTS))
                .insert(LightOccluder2D {
                    h_size: Vec2::splat(2.0),
                }) */
                .id();
        };

        let mut hex_pos = hex(8, -12);
        let mut world_pos = HEX_LAYOUT.hex_to_world_pos(hex_pos);

        lights.push(spawn_light(
            &mut commands,
            world_pos.x,
            world_pos.y,
            "rop",
            OmniLightSource2D {
                intensity: 0.8,
                color: Color::rgb_u8(0, 255, 0),
                falloff: Vec3::new(1.5, 10.0, 0.005),
                ..default()
            },
        ));

        hex_pos = hex(-8, 12);
        world_pos = HEX_LAYOUT.hex_to_world_pos(hex_pos);

        lights.push(spawn_light(
            &mut commands,
            world_pos.x,
            world_pos.y,
            "rop",
            OmniLightSource2D {
                intensity: 0.8,
                color: Color::rgb_u8(0, 255, 0),
                falloff: Vec3::new(1.5, 10.0, 0.005),
                ..default()
            },
        ));
    }

    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("lights"))
        .push_children(&lights);

    // skylight

    commands.spawn((
        SkylightLight2D {
            color: Color::rgb_u8(93, 158, 179),
            intensity: 0.025,
        },
        Name::new("global_skylight"),
    ));
}
