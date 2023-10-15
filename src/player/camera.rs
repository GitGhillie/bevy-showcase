use bevy::prelude::*;

use bevy::ecs::system::EntityCommands;
use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasBundle,
    pbr::ScreenSpaceAmbientOcclusionBundle,
};

pub trait CameraComponents {
    fn insert_camera_components(&mut self) -> &mut Self;
}

impl CameraComponents for EntityCommands<'_, '_, '_> {
    fn insert_camera_components(&mut self) -> &mut Self {
        self.insert(Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 90.0 * (std::f32::consts::PI / 180.0),
                aspect_ratio: 1.0,
                near: 0.3,
                far: 1000.0,
            }),
            ..default()
        })
        .insert(TemporalAntiAliasBundle::default())
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
    }
}
