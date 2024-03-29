use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{Player, PLAYER_SIZE};

pub fn limit_player_movement(
    mut player: Query<&mut Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();
    let half_player_size = PLAYER_SIZE / 2.;

    if let Ok(mut player_transform) = player.get_single_mut() {
        let mut translation = player_transform.translation;

        translation.x = translation.x.max(half_player_size);
        translation.x = translation.x.min(window.width() - half_player_size);
        translation.y = translation.y.max(half_player_size);
        translation.y = translation.y.min(window.height() - half_player_size);

        player_transform.translation = translation;
    }
}
