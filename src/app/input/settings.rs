use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(Resource)]
pub(crate) struct InputSettings {
    pub forward: [Binding; 2],
    pub right: [Binding; 2],
    pub backward: [Binding; 2],
    pub left: [Binding; 2],
    pub up: [Binding; 2],
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            forward: [Binding::from(KeyCode::KeyW), Binding::None],
            right: [Binding::from(KeyCode::KeyD), Binding::None],
            backward: [Binding::from(KeyCode::KeyS), Binding::None],
            left: [Binding::from(KeyCode::KeyA), Binding::None],
            up: [Binding::from(KeyCode::Space), Binding::None],
        }
    }
}
