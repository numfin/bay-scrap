use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::{Enemy, ENEMY_SIZE};

pub fn update_enemy_rotation(
    mut enemies: Query<(&Transform, &mut Enemy)>,
    window: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();

    for (transform, mut enemy) in &mut enemies {
        let translation = transform.translation;
        let enemy_size_half = ENEMY_SIZE / 2.;
        let mut direction_changed = false;

        if translation.x > window.width() - enemy_size_half {
            enemy.direction.x = -1.;
            direction_changed = true;
        }
        if translation.x < enemy_size_half {
            enemy.direction.x = 1.;
            direction_changed = true;
        }
        if translation.y > window.height() - enemy_size_half {
            enemy.direction.y = -1.;
            direction_changed = true;
        }
        if translation.y < enemy_size_half {
            enemy.direction.y = 1.;
            direction_changed = true;
        }

        if direction_changed {
            let sfx_list = [
                // asset_server.load("audio/impactMetal_000.ogg"),
                asset_server.load("audio/impactMetal_001.ogg"),
            ];
            if let Some(sfx) = sfx_list.choose(&mut thread_rng()) {
                audio.play(sfx.to_owned());
            }
        }
    }
}
