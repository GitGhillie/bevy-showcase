// Copied from Kaosat-dev on Github
pub mod physics_replace_proxies;
pub use physics_replace_proxies::*;

pub mod utils;

use bevy::prelude::*;

pub struct ReplaceColliderPlugin;

impl Plugin for ReplaceColliderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AutoAABBCollider>()
            .register_type::<Collider>()
            .add_systems(Update, physics_replace_proxies);
    }
}
