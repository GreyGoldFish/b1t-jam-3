use bevy::{prelude::*, window::CursorOptions};
use bevy_enhanced_input::prelude::*;

use crate::app::{
    input::actions::RotationAction,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(apply_rotation_action);
}

fn apply_rotation_action(
    rotate_event: On<Fire<RotationAction>>,
    mut transforms: Query<&mut Transform>,
    cursor_options: Single<&CursorOptions>,
) {
    if cursor_options.visible {
        return; // Don't rotate if cursor is visible
    }

    let mut transform = transforms.get_mut(rotate_event.context).unwrap();
    let (mut yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);

    yaw += rotate_event.value.x.to_radians();

    // Only apply yaw rotation (Y axis)
    transform.rotation = Quat::from_rotation_y(yaw);
}
