use bevy::prelude::*;

mod controller;

use crate::app::character::PlayerControlled;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        controller::plugin,
    ));
    app.add_observer(add_camera_to_player);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct PitchPivot;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Camera3d)]
pub struct MainCamera;

fn add_camera_to_player(
    controlled_by_player: On<Add, PlayerControlled>,
    mut commands: Commands,
) {
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

    commands
        .entity(pitch)
        .add_child(camera);

    commands
        .entity(controlled_by_player.entity)
        .add_child(pitch);
}
