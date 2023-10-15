use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

pub struct AttractPlugin;

impl Plugin for AttractPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, attract);
    }
}

fn attract(mut ext_forces: Query<(&mut ExternalForce, &mut GlobalTransform)>) {
    for (mut ext_force, transform) in ext_forces.iter_mut() {
        let attraction_point = Vec3::new(0.0, 0.0, 0.0);
        let source = transform.translation();

        let attraction_vector = attraction_point - source;

        ext_force.force = attraction_vector.normalize_or_zero() * 3000.0;
    }
}
