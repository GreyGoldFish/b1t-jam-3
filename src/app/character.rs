use bevy::prelude::*;

pub mod controller;
pub mod control;

pub use control::PlayerControlled;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        controller::plugin,
        control::plugin,
    ));
}
