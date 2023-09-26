use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_mod_wanderlust::{
    ControllerBundle, ControllerInput, ControllerPhysicsBundle, ControllerSettings,
    WanderlustPlugin,
};
use bevy_rapier3d::prelude::*;

use std::f32::consts::FRAC_2_PI;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(WanderlustPlugin)
            .insert_resource(Sensitivity(1.0))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    movement_input.before(bevy_mod_wanderlust::movement),
                    mouse_look,
                    toggle_cursor_lock,
                ),
            );
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct PlayerCam;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct PlayerBody;

#[derive(Reflect, Resource)]
struct Sensitivity(f32);

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
            PlayerBody,
        ))
        .insert(PbrBundle {
            mesh,
            material: material.clone(),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn((
                    Camera3dBundle {
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
                        projection: Projection::Perspective(PerspectiveProjection {
                            fov: 90.0 * (std::f32::consts::PI / 180.0),
                            aspect_ratio: 1.0,
                            near: 0.3,
                            far: 1000.0,
                        }),
                        ..default()
                    },
                    PlayerCam,
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

fn movement_input(
    mut body: Query<&mut ControllerInput, With<PlayerBody>>,
    camera: Query<&GlobalTransform, (With<PlayerCam>, Without<PlayerBody>)>,
    input: Res<Input<KeyCode>>,
) {
    let tf = camera.single();

    let mut player_input = body.single_mut();

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::A) {
        dir += -tf.right();
    }
    if input.pressed(KeyCode::D) {
        dir += tf.right();
    }
    if input.pressed(KeyCode::S) {
        dir += -tf.forward();
    }
    if input.pressed(KeyCode::W) {
        dir += tf.forward();
    }
    dir.y = 0.0;
    player_input.movement = dir.normalize_or_zero();

    player_input.jumping = input.pressed(KeyCode::Space);
}

fn mouse_look(
    mut cam: Query<&mut Transform, With<PlayerCam>>,
    mut body: Query<&mut Transform, (With<PlayerBody>, Without<PlayerCam>)>,
    sensitivity: Res<Sensitivity>,
    mut input: EventReader<MouseMotion>,
) {
    let mut cam_tf = cam.single_mut();
    let mut body_tf = body.single_mut();

    let sens = sensitivity.0;

    let mut cumulative: Vec2 = -(input.iter().map(|motion| &motion.delta).sum::<Vec2>());

    // Vertical
    let rot = cam_tf.rotation;

    // Ensure the vertical rotation is clamped
    if rot.x > FRAC_2_PI && cumulative.y.is_sign_positive()
        || rot.x < -FRAC_2_PI && cumulative.y.is_sign_negative()
    {
        cumulative.y = 0.0;
    }

    cam_tf.rotate(Quat::from_scaled_axis(
        rot * Vec3::X * cumulative.y / 180.0 * sens,
    ));

    // Horizontal
    let rot = body_tf.rotation;
    body_tf.rotate(Quat::from_scaled_axis(
        rot * Vec3::Y * cumulative.x / 180.0 * sens,
    ));
}

fn toggle_cursor_lock(
    input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        match window.cursor.grab_mode {
            CursorGrabMode::Locked => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
            _ => {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }
}
