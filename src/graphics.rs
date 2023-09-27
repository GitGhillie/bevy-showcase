use bevy::pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder};
use bevy::prelude::*;

pub(crate) fn get_fog_settings() -> FogSettings {
    FogSettings {
        color: Color::rgba(0.1, 0.2, 0.4, 1.0),
        directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
        directional_light_exponent: 30.0,
        falloff: FogFalloff::from_visibility_colors(
            90.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
            Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
            Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
        ),
    }
}

pub(crate) fn create_cascade_shadow_config() -> CascadeShadowConfig {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 30.0,
        ..default()
    }
    .build();

    cascade_shadow_config
}
