use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::{Enemy, ENEMIES_AMOUNT};

pub fn spawn_enemies(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();
    for _ in 0..ENEMIES_AMOUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new(random(), random()).normalize(),
            },
        ));
    }
}
