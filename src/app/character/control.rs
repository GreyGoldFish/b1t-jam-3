use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct ControlledByPlayer;

pub(super) fn plugin(app: &mut App) {
    app
        .register_type::<ControlledByPlayer>();
}
