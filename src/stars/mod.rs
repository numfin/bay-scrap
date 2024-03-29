use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use self::collect_stars::{player_hit_star, Score};
use self::spawn_over_time::{spawn_over_time, StarSpawnTimer};

pub mod collect_stars;
pub mod spawn_over_time;

pub const STARS_AMOUNT: usize = 5;
pub const STAR_SIZE: f32 = 30.;
pub const STAR_SPAWN_SECS: f32 = 1.;

pub struct StarsPlugin;
impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_system(player_hit_star)
            .add_system(spawn_over_time);
    }
}

#[derive(Component)]
pub struct Star {}

fn spawn_stars(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();

    for _ in 0..STARS_AMOUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}
