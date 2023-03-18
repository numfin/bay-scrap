use bevy::prelude::*;

use crate::system_sets::{LimitMovementSystemSet, MovementSystemSet};

use self::enemy_collision::enemy_collision;
use self::limit_player_movement::limit_player_movement;
use self::player_movement::player_movement;
use self::spawn_player::spawn_player;

mod enemy_collision;
mod limit_player_movement;
mod player_movement;
mod spawn_player;

pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement.in_set(MovementSystemSet))
            .add_system(limit_player_movement.in_set(LimitMovementSystemSet))
            .add_system(enemy_collision.in_set(LimitMovementSystemSet));
    }
}

#[derive(Component)]
pub struct Player {}
