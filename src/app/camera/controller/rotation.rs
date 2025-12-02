use bevy::{prelude::*, window::CursorOptions};
use bevy_enhanced_input::prelude::*;

use crate::app::{
    camera::PitchPivot, character::PlayerControlled, input::actions::RotationAction
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_camera_pitch);
}

fn apply_camera_pitch(
    rotation_actions: Query<&Action<RotationAction>>,
    players: Query<(&Actions<PlayerControlled>, &Children)>,
    mut pitch_pivots: Query<&mut Transform, With<PitchPivot>>,
    cursor_options: Single<&CursorOptions>,
) {
    if cursor_options.visible { return; }

    for (actions, children) in &players {
        if let Some(rotation) = rotation_actions.iter_many(actions).next() {
            for child in children.iter() {
                if let Ok(mut transform) = pitch_pivots.get_mut(child) {
                    let (_, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                    pitch += rotation.y.to_radians();
                    transform.rotation = Quat::from_rotation_x(pitch.clamp(-1.54, 1.54));
                }
            }
        }
    }
}
