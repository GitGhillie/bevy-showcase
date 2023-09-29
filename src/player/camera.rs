use bevy::prelude::*;

pub(crate) fn create_camera_bundle() -> Camera3dBundle {
    Camera3dBundle {
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
    }
}
