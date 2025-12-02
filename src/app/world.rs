use bevy::prelude::*;

mod level;

pub use level::spawn_level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
    ));
}
