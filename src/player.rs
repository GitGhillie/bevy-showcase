use bevy::prelude::*;

use bevy_xpbd_3d::{math::*, prelude::*, PhysicsSchedule, PhysicsStepSet};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .add_systems(Startup, setup)
            .add_systems(PhysicsSchedule, movement.before(PhysicsStepSet::BroadPhase));
    }
}

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.4,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        },
        RigidBody::Dynamic,
        Position(Vector::Y * 1.0),
        Collider::capsule(1.0, 0.4),
        // Prevent the player from falling over
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        // Cast the player shape downwards to detect when the player is grounded
        ShapeCaster::new(
            Collider::capsule(0.9, 0.35),
            Vector::NEG_Y * 0.05,
            Quaternion::default(),
            Vector::NEG_Y,
        )
        .with_ignore_origin_penetration(true) // Don't count player's collider
        .with_max_time_of_impact(0.2)
        .with_max_hits(1),
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
        Player,
    ));
}

#[derive(Component)]
struct Player;

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut LinearVelocity, &ShapeHits), With<Player>>,
) {
    for (mut linear_velocity, ground_hits) in &mut players {
        // Directional movement
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            linear_velocity.z -= 1.2;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            linear_velocity.z += 1.2;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            linear_velocity.x -= 1.2;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            linear_velocity.x += 1.2;
        }

        // Jump if space pressed and the player is close enough to the ground
        if keyboard_input.just_pressed(KeyCode::Space) && !ground_hits.is_empty() {
            linear_velocity.y += 8.0;
        }

        // Slow player down on the x and y axes
        linear_velocity.x *= 0.8;
        linear_velocity.z *= 0.8;
    }
}

// fn kinematic_collision(
//     collisions: Res<Collisions>,
//     mut bodies: Query<(&RigidBody, &mut Position, &Rotation)>,
// ) {
//     // Iterate through collisions and move the kinematic body to resolve penetration
//     for contacts in collisions.iter() {
//         // If the collision didn't happen during this substep, skip the collision
//         if !contacts.during_current_substep {
//             continue;
//         }
//         if let Ok([(rb1, mut position1, rotation1), (rb2, mut position2, _)]) =
//             bodies.get_many_mut([contacts.entity1, contacts.entity2])
//         {
//             for manifold in contacts.manifolds.iter() {
//                 for contact in manifold.contacts.iter() {
//                     if contact.penetration <= Scalar::EPSILON {
//                         continue;
//                     }
//                     if rb1.is_kinematic() && !rb2.is_kinematic() {
//                         position1.0 -= contact.global_normal1(rotation1) * contact.penetration;
//                     } else if rb2.is_kinematic() && !rb1.is_kinematic() {
//                         position2.0 += contact.global_normal1(rotation1) * contact.penetration;
//                     }
//                 }
//             }
//         }
//     }
// }
