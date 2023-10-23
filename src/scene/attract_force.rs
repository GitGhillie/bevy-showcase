use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

#[derive(Component)]
pub struct AttractMarker;

pub struct AttractPlugin;

impl Plugin for AttractPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (attract, release_attract).chain());
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

        // if mouse_button_input.pressed(MouseButton::Left) {
        ext_force.force = attraction_vector.normalize_or_zero() * 300.0;
        // } else {
        //     ext_force.force = Vec3::ZERO;
        // }
    }
}

// Todo release left mouse button removes all attract markers
fn release_attract(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut attracted_entities: Query<(Entity, &mut ExternalForce), With<AttractMarker>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        for (ent, mut ext_force) in attracted_entities.iter_mut() {
            println!("Release!");
            ext_force.force = Vec3::ZERO;
            commands.entity(ent).remove::<AttractMarker>();
        }
    }
}
