use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

#[derive(Component)]
pub struct AttractMarker;

pub struct AttractPlugin;

impl Plugin for AttractPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, attract)
            .add_systems(Update, release_attract);
    }
}

fn attract(
    //mouse_button_input: Res<Input<MouseButton>>,
    cam_transform: Query<&GlobalTransform, With<Camera>>,
    mut ext_forces: Query<
        (&mut ExternalForce, &mut GlobalTransform),
        (Without<Camera>, With<AttractMarker>),
    >,
) {
    let cam_transform = cam_transform.get_single().ok().unwrap();

    for (mut ext_force, transform) in ext_forces.iter_mut() {
        let attraction_point = cam_transform.translation() + (cam_transform.forward() * 5.0);
        let source = transform.translation();

        let attraction_vector = attraction_point - source;

        ext_force.force = attraction_vector.normalize_or_zero() * 300.0;
    }
}

fn release_attract(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut attracted_entities: Query<(Entity, &mut ExternalForce), With<AttractMarker>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        for (ent, mut ext_force) in attracted_entities.iter_mut() {
            ext_force.force = Vec3::ZERO;
            commands.entity(ent).remove::<AttractMarker>();
        }
    }
}
