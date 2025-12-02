use bevy::prelude::*;

mod rotation;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        rotation::plugin,
    ));
}
