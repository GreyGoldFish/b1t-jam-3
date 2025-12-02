use bevy::prelude::*;
use avian3d::{math::*, prelude::*};

use crate::app::character::{ControlledByPlayer, controller::CharacterControllerBundle};

pub(super) fn plugin(app: &mut App) {

}

pub fn spawn_level(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    //let level_scene: Handle<Scene> =
        //asset_server.load(GltfAssetLabel::Scene(0).from_asset("level0.glb"));

    //commands.spawn(SceneRoot(level_scene));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(8.0, 8.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        ColliderConstructor::TrimeshFromMesh,
        RigidBody::Static,
    ));

    // A cube to move around
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(3.0, 2.0, 3.0),
    ));

    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 50.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 15.0, 0.0),
    ));

    spawn_player(commands);
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        ControlledByPlayer,
        Transform::from_xyz(0.0, 1.5, 0.0),
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
            .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
    ));
}
