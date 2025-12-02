use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_enhanced_input::prelude::*;

pub mod actions;
pub mod settings;

use actions::{
    WalkAction,
    JumpAction,
    RotationAction,
    CaptureCursor,
    ReleaseCursor,
};

use settings::InputSettings;

use crate::app::character::PlayerControlled;

pub(super) fn plugin(app: &mut App) {
    app
        .init_resource::<InputSettings>()
        .add_systems(Startup, setup)
        .add_plugins(EnhancedInputPlugin)
        .add_input_context::<PlayerControlled>()
        .add_observer(apply_player_actions)
            .add_observer(on_capture_cursor_completed)
            .add_observer(on_release_cursor_completed);
}

fn setup(
    mut cursor_options: Single<&mut CursorOptions>,
) {
    grab_cursor(&mut cursor_options, true);
}

fn apply_player_actions(
    controlled_by_player: On<Add, PlayerControlled>,
    settings: Res<InputSettings>,
    mut commands: Commands,
) {
    let player = controlled_by_player.entity;

    let move_action = (
        Action::<WalkAction>::new(),
        DeadZone::default(), // Applies non-uniform normalization
        SmoothNudge::default(), // Smoothes movement
        Scale::splat(0.3), // TODO: remove magic number
        Bindings::spawn((
            Cardinal {
                north: settings.forward[0],
                east: settings.right[0],
                south: settings.backward[0],
                west: settings.left[0],
            },
            Cardinal {
                north: settings.forward[1],
                east: settings.right[1],
                south: settings.backward[1],
                west: settings.left[1],
            },
            Axial::left_stick(),
        ))
    );

    let jump_action = (
        Action::<JumpAction>::new(),
        Bindings::spawn((
            Spawn((settings.up[0],)),
            Spawn((settings.up[1],)),
        ))
    );

    let rotation_action = (
        Action::<RotationAction>::new(),
        Bindings::spawn((
            Spawn((
                Binding::mouse_motion(),
                Scale::splat(0.1), // TODO: remove magic number
                Negate::all(),
            )),
            Axial::right_stick().with((
                Scale::splat(2.0), // TODO: remove magic number
                Negate::x(),
            )),
        ))
    );

    commands.entity(player).insert(actions!(PlayerControlled[
        move_action,
        rotation_action,
        jump_action,
    ]));
}

fn on_capture_cursor_completed(
    _on: On<Complete<CaptureCursor>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    grab_cursor(&mut cursor_options, true);
}

fn on_release_cursor_completed(
    _on: On<Complete<ReleaseCursor>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    grab_cursor(&mut cursor_options, false);
}

fn grab_cursor(cursor_options: &mut CursorOptions, grab: bool) {
    cursor_options.grab_mode = if grab {
        CursorGrabMode::Confined
    } else {
        CursorGrabMode::None
    };
    cursor_options.visible = !grab;
}
