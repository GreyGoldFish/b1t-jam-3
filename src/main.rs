use bevy::prelude::*;
use bevy_skein::SkeinPlugin;
use avian3d::prelude::PhysicsPlugins;

mod app;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            app::AppPlugin,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
        ))
        .run()
}
