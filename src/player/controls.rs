use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_mod_wanderlust::ControllerInput;

use std::f32::consts::FRAC_2_PI;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct PlayerCam;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct PlayerBody;

#[derive(Reflect, Resource)]
pub(crate) struct Sensitivity(pub f32);

pub(crate) fn movement_input(
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

pub(crate) fn mouse_look(
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
