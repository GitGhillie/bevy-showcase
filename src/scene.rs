mod ambient_sound;
mod attract_force;
mod audio;
mod police_cars;
mod spawn_objects;
mod trains;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::graphics;

pub struct SceneLoader;

impl Plugin for SceneLoader {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            brightness: 0.3,
            ..default()
        })
        .add_plugins(ComponentsFromGltfPlugin)
        .add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>()
                .disable::<DebugPickingPlugin>(),
        )
        .add_plugins((
            audio::InsertAudioPlugin,
            ambient_sound::AmbientSoundPlugin,
            police_cars::PoliceCarPlugin,
            trains::TrainsPlugin,
            attract_force::AttractPlugin,
            spawn_objects::SpawnObjectsPlugin,
        ))
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Next),
        )
        .add_collection_to_loading_state::<_, SceneAssets>(GameState::AssetLoading)
        .add_systems(OnEnter(GameState::Next), spawn_scene)
        .add_systems(Startup, setup);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "level.glb#Scene0")]
    scene: Handle<Scene>,
    #[asset(path = "detail.glb#Scene0")]
    detail: Handle<Scene>,
    // Below are random objects for spawn_objects
    #[asset(path = "objects/trashcan.glb#Scene0")]
    trashcan: Handle<Scene>,
    #[asset(path = "objects/police_car.glb#Scene0")]
    police_car: Handle<Scene>,
    #[asset(path = "objects/tire.glb#Scene0")]
    tire: Handle<Scene>,
    #[asset(path = "objects/pallet.glb#Scene0")]
    pallet: Handle<Scene>,
}

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut physics_config: ResMut<RapierConfiguration>,
) {
    // Disable physics while assets are loading
    physics_config.physics_pipeline_active = false;

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(0.15, -0.35, -0.25), Vec3::Y),
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
            transform: Transform::from_scale(Vec3::splat(1600.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

fn spawn_scene(
    mut commands: Commands,
    my_assets: Res<SceneAssets>,
    mut physics_config: ResMut<RapierConfiguration>,
) {
    commands.spawn((
        SceneBundle {
            scene: my_assets.scene.clone_weak(),
            ..default()
        },
        AsyncSceneCollider::default(),
        Name::from("Blockout"),
    ));

    commands.spawn((
        SceneBundle {
            scene: my_assets.detail.clone_weak(),
            ..default()
        },
        Name::from("Detail"),
    ));

    physics_config.physics_pipeline_active = true;
}
