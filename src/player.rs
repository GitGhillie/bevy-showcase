use bevy::prelude::*;
use bevy_fmod::prelude::AudioListener;
use bevy_mod_picking::prelude::RaycastPickCamera;
use bevy_mod_wanderlust::{
    ControllerBundle, ControllerPhysicsBundle, ControllerSettings, WanderlustPlugin,
};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

mod camera;
mod controls;

use crate::graphics;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            //.add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(WanderlustPlugin)
            .insert_resource(controls::Sensitivity(0.5))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    controls::movement_input.before(bevy_mod_wanderlust::movement),
                    controls::mouse_look,
                    controls::toggle_cursor_lock,
                ),
            );
    }
}

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(
        shape::Capsule {
            radius: 0.5,
            depth: 1.0,
            ..default()
        }
        .into(),
    );

    let material = materials.add(Color::WHITE.into());

    commands
        .spawn((
            ControllerBundle {
                settings: ControllerSettings::character(),
                physics: ControllerPhysicsBundle {
                    // Lock the axes to prevent camera shake whilst moving up slopes
                    locked_axes: LockedAxes::ROTATION_LOCKED,
                    ..default()
                },
                ..default()
            },
            Name::from("Player"),
            controls::PlayerBody,
        ))
        .insert(PbrBundle {
            mesh,
            material: material.clone(),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn((
                    camera::create_camera_bundle(),
                    AudioListener::default(),
                    Velocity::default(),
                    graphics::get_fog_settings(),
                    RaycastPickCamera::default(),
                    controls::PlayerCam,
                ))
                .with_children(|commands| {
                    let mesh = meshes.add(shape::Cube { size: 0.5 }.into());

                    commands.spawn(PbrBundle {
                        mesh,
                        material: material.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, -0.5),
                        ..default()
                    });
                });
        });
}
