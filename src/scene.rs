use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::graphics;

pub struct SceneLoader;

impl Plugin for SceneLoader {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Next),
            )
            .add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading)
            .add_systems(OnEnter(GameState::Next), use_my_assets)
            .add_systems(Startup, setup);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "level.glb#Scene0")]
    scene: Handle<Scene>,
}

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(8.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Collider::cuboid(4.0, 0.005, 4.0),
    ));

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config: graphics::create_cascade_shadow_config(),
        ..default()
    });

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(80.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

fn use_my_assets(mut commands: Commands, my_assets: Res<MyAssets>) {
    commands.spawn((
        SceneBundle {
            scene: my_assets.scene.clone_weak(),
            ..default()
        },
        AsyncSceneCollider::default(),
    ));
}
