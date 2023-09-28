// Copied from Kaosat-dev on Github

use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider as RapierCollider;
use bevy_rapier3d::prelude::{ActiveEvents, ComputedColliderShape};

use super::utils::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum Collider {
    Ball(f32),
    Cuboid(Vec3),
    Capsule(Vec3, Vec3, f32),
    #[default]
    Mesh,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum AutoAABBCollider {
    #[default]
    Cuboid,
    Ball,
    Capsule,
}

// replaces all physics stand-ins with the actual rapier types
pub fn physics_replace_proxies(
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut proxy_colliders: Query<
        (Entity, &Collider, &Name, &mut Visibility),
        (Without<RapierCollider>, Added<Collider>),
    >,
    // needed for tri meshes
    children: Query<&Children>,

    mut commands: Commands,
) {
    for proxy_collider in proxy_colliders.iter_mut() {
        let (entity, collider_proxy, name, mut visibility) = proxy_collider;
        // we hide the collider meshes: perhaps they should be removed altogether once processed ?
        if name.ends_with("_collider") || name.ends_with("_sensor") {
            *visibility = Visibility::Hidden;
        }

        let mut rapier_collider: RapierCollider;
        match collider_proxy {
            Collider::Ball(radius) => {
                println!("proxy: ball");
                rapier_collider = RapierCollider::ball(*radius);
                commands.entity(entity)
                    .insert(rapier_collider)
                    .insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes !!!
                ;
            }
            Collider::Cuboid(size) => {
                println!("proxy: cuboid");
                rapier_collider = RapierCollider::cuboid(size.x, size.y, size.z);
                commands.entity(entity)
                    .insert(rapier_collider)
                    .insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes !!!
                ;
            }
            Collider::Capsule(a, b, radius) => {
                println!("proxy: capsule");
                rapier_collider = RapierCollider::capsule(*a, *b, *radius);
                commands.entity(entity)
                    .insert(rapier_collider)
                    .insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes !!!
                ;
            }
            Collider::Mesh => {
                println!("proxy: mesh");
                for (_, collider_mesh) in
                    Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
                {
                    rapier_collider = RapierCollider::from_bevy_mesh(
                        collider_mesh,
                        &ComputedColliderShape::ConvexHull,
                    )
                    .unwrap();
                    commands.entity(entity).insert(rapier_collider);
                    // FIXME: this is just for demo purposes !!!
                    // .insert(
                    //     ActiveCollisionTypes::default()
                    //         | ActiveCollisionTypes::KINEMATIC_STATIC
                    //         | ActiveCollisionTypes::STATIC_STATIC
                    //         | ActiveCollisionTypes::DYNAMIC_STATIC,
                    // )
                    // .insert(ActiveEvents::COLLISION_EVENTS);
                    //  .insert(ActiveEvents::COLLISION_EVENTS)
                    // break;
                    // RapierCollider::convex_hull(points)
                }
            }
        }
    }
}
