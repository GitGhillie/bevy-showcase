use bevy::prelude::*;
use bevy_rapier3d::prelude::{Damping, ExternalForce, RigidBody};

#[derive(Component)]
pub struct AttractMarker;

pub struct AttractPlugin;

impl Plugin for AttractPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, attract)
            .add_systems(Update, (release_attract, propagate_attract_marker));
    }
}

fn attract(
    cam_transform: Query<&GlobalTransform, With<Camera>>,
    mut ext_forces: Query<
        (&mut ExternalForce, &mut GlobalTransform, &mut Damping),
        (Without<Camera>, With<AttractMarker>),
    >,
) {
    let cam_transform = cam_transform.get_single().ok().unwrap();

    for (mut ext_force, global_transform, mut damping) in ext_forces.iter_mut() {
        let attraction_point = cam_transform.translation() + (cam_transform.forward() * 3.0);
        let source = global_transform.translation();

        let attraction_vector = attraction_point - source;

        let distance = attraction_vector.length();
        let attraction_force: f32;

        if distance > 1.0 {
            attraction_force = 3000.0 / attraction_vector.length();
            damping.linear_damping = 0.0;
            damping.angular_damping = 0.0;
        } else {
            attraction_force = 3000.0;
            damping.linear_damping = 60.0;
            damping.angular_damping = 1.0;
        }

        ext_force.force = attraction_vector.normalize_or_zero() * attraction_force;
    }
}

fn release_attract(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut attracted_entities: Query<(Entity, &mut ExternalForce, &mut Damping), With<AttractMarker>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        for (ent, mut ext_force, mut damping) in attracted_entities.iter_mut() {
            ext_force.force = Vec3::ZERO;
            damping.linear_damping = 0.0;
            damping.angular_damping = 0.0;
            commands.entity(ent).remove::<AttractMarker>();
        }
    }
}

// Because the attract marker gets inserted on the child mesh that was clicked (bug?)
// we have to propagate it up to the 'top-level' entity.
fn propagate_attract_marker(
    mut commands: Commands,
    child_query: Query<(Entity, &Parent), With<AttractMarker>>,
    parent_query: Query<Entity, (Without<AttractMarker>, With<RigidBody>)>,
) {
    for (child, childs_parent) in &child_query {
        if let Ok(parent) = parent_query.get(childs_parent.get()) {
            commands.entity(parent).insert(AttractMarker);
            commands.entity(child).remove::<AttractMarker>();
        }
    }
}
