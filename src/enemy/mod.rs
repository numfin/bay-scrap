use bevy::prelude::*;

use crate::system_sets::{LimitMovementSystemSet, MovementSystemSet};

use self::enemy_movement::enemy_movement;
use self::limit_enemy_movement::limit_enemy_movement;
use self::spawn_enemies::spawn_enemies;
use self::update_enemy_rotation::update_enemy_rotation;

mod enemy_movement;
mod limit_enemy_movement;
mod spawn_enemies;
mod update_enemy_rotation;

pub const ENEMIES_AMOUNT: usize = 4;
pub const ENEMY_SPEED: f32 = 40.;
pub const ENEMY_SIZE: f32 = 64.;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemies)
            .add_system(enemy_movement.in_set(MovementSystemSet))
            .add_system(limit_enemy_movement.in_set(LimitMovementSystemSet))
            .add_system(update_enemy_rotation.after(LimitMovementSystemSet));
    }
}

#[derive(Component)]
pub struct Enemy {
    direction: Vec2,
}
