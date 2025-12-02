use bevy::prelude::*;

use crate::app::character::ControlledByPlayer;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_camera_to_player);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct YawPivot;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct PitchPivot;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Camera3d)]
pub struct MainCamera;

#[derive(Debug, Copy, Clone)]
struct CameraRig {
    pub yaw: Entity,
    pub pitch: Entity,
    pub camera: Entity,
}

fn add_camera_to_player(
    controlled_by_player: On<Add, ControlledByPlayer>,
    mut commands: Commands,
) {
    let camera_rig = spawn_camera_rig(&mut commands);
    commands
        .entity(controlled_by_player.entity)
        .add_child(camera_rig.yaw);
}

fn spawn_camera_rig(commands: &mut Commands) -> CameraRig {
    let yaw = commands
        .spawn((
            Name::new("Yaw Pivot"),
            YawPivot,
        ))
        .id();

    let pitch = commands
        .spawn((
            Name::new("Pitch Pivot"),
            PitchPivot,
        ))
        .id();

    let camera = commands
        .spawn((
            Name::new("Main Camera"),
            MainCamera,
        ))
        .id();

    commands.entity(yaw).add_child(pitch);
    commands.entity(pitch).add_child(camera);

    CameraRig { yaw, pitch, camera }
}
