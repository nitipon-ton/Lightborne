use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Player;

pub fn move_player(
    mut query: Query<(
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &mut Player,
    )>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut controller, output, mut player)) = query.get_single_mut() else {
        return;
    };

    const PLAYER_MAX_H_VEL: f32 = 3.;
    const PLAYER_MAX_Y_VEL: f32 = 5.;
    const PLAYER_AIR_MOVEMENT_COEFF: f32 = 0.2;

    // Use pressed instead of just_pressed because an object on the ground is not grounded every
    // frame??
    let mut jumped = false;
    if keys.pressed(KeyCode::Space) && output.grounded {
        player.velocity.y = 4.;
        jumped = true;
    } else if !keys.pressed(KeyCode::Space) && player.velocity.y > 0. {
        // Jump cutting
        // TODO: ensure minimum length jump
        player.velocity.y = 0.;
    } else if output.grounded {
        player.velocity.y = 0.;
    }
    player.velocity.y = player.velocity.y.clamp(-PLAYER_MAX_Y_VEL, PLAYER_MAX_Y_VEL);
    if !jumped {
        // Gravity
        player.velocity.y -= 0.2;
    }

    let mut delta_v_h = 1.;
    if !output.grounded {
        delta_v_h *= PLAYER_AIR_MOVEMENT_COEFF
    };
    let mut moved = false;
    if keys.pressed(KeyCode::KeyA) {
        // Change dirs instantly
        if player.velocity.x > 0. {
            player.velocity.x = 0.;
        }
        player.velocity.x -= delta_v_h;
        moved = true;
    }
    if keys.pressed(KeyCode::KeyD) {
        // Change dirs instantly
        if player.velocity.x < 0. {
            player.velocity.x = 0.;
        }
        player.velocity.x += delta_v_h;
        moved = true;
    }
    player.velocity.x = player.velocity.x.clamp(-PLAYER_MAX_H_VEL, PLAYER_MAX_H_VEL);
    if !moved {
        // slow player down when not moving horizontally
        // NOTE: why not using rapier friction?
        player.velocity.x *= 0.6;
        if player.velocity.x.abs() < 0.1 {
            player.velocity.x = 0.;
        }
    }

    controller.translation = Some(player.velocity);
}
