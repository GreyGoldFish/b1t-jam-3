use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::{
    prelude::*,
    math::*,
};

use super:: {
    MovementAcceleration,
    MovementDampingFactor,
    JumpImpulse,
    MaxSlopeAngle,
    Grounded,
};

use crate::app::{
    character::ControlledByPlayer,
    input::actions::{
        JumpAction,
        WalkAction,
    },
    camera::YawPivot,
};

pub(super) fn plugin(app: &mut App) {
    app
        .add_systems(
            Update,
            (
                apply_movement.in_set(MovementSet),
                apply_movement_damping.in_set(MovementSet),
            ).chain(),
        );
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSet;

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

// Applies movement and jumping based on actions.
fn apply_movement(
    jump_events: Query<&ActionEvents, With<Action<JumpAction>>>,
    walk_actions: Query<&Action<WalkAction>>,
    time: Res<Time>,
    children_of: Query<&Children>,
    yaw_globals: Query<&GlobalTransform, With<YawPivot>>,
    mut controllers: Query<(
        Entity,
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
        &Actions<ControlledByPlayer>
    )>,
) {
    for (entity, movement_acceleration, jump_impulse, mut linear_velocity, is_grounded, actions) in
        &mut controllers
    {
        if !is_grounded {
            continue;
        }

        if let Some(jump_events) = jump_events.iter_many(actions).next() {
            if jump_events.contains(ActionEvents::STARTED) {
                linear_velocity.y += jump_impulse.value();
            }
        }

        if let Some(walk_action) = walk_actions.iter_many(actions).next() {
            let delta_time = time.delta_secs();

            // Local input (match existing mapping: forward is -Z)
            let local = Vec3::new(walk_action.x, 0.0, -walk_action.y);

            // Try to rotate by the player's yaw pivot
            let world = if let Ok(children) = children_of.get(entity) {
                // Find the YawPivot among the player's children
                let mut rotated = None;
                for child in children.iter() {
                    if let Ok(gt) = yaw_globals.get(child) {
                        let (_, yaw_rot, _) = gt.to_scale_rotation_translation();
                        rotated = Some(yaw_rot * local);
                        break;
                    }
                }
                rotated.unwrap_or(local)
            } else {
                local
            };

            // Apply movement in world space respecting yaw
            linear_velocity.x += world.x * movement_acceleration.value() * delta_time;
            linear_velocity.z += world.z * movement_acceleration.value() * delta_time;
        }
    }
}

/// Slows down movement in the XZ plane.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.value();
        linear_velocity.z *= damping_factor.value();
    }
}
