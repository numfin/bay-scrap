use bevy::prelude::*;

use crate::player::{Player, PLAYER_SIZE};

use super::{Star, STAR_SIZE};

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

pub fn player_hit_star(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    enemies: Query<(Entity, &Transform), With<Star>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player.get_single() {
        let min_distance = (PLAYER_SIZE + STAR_SIZE) / 2.;

        for (enemy_entity, enemy_transform) in &enemies {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < min_distance {
                let sfx = asset_server.load("audio/laserRetro_004.ogg");
                audio.play(sfx);
                commands.entity(enemy_entity).despawn();
                score.value += 1;
            }
        }
    }
}
