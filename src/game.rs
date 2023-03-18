use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::controls::ControlsPlugin;
use crate::enemy::EnemyPlugin;
use crate::game_over::GameOverPlugin;
use crate::player::PlayerPlugin;
use crate::stars::StarsPlugin;
use crate::system_sets::{LimitMovementSystemSet, MovementSystemSet};

pub fn run() {
    App::new()
        .configure_set(LimitMovementSystemSet.after(MovementSystemSet))
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(StarsPlugin)
        .add_plugin(GameOverPlugin)
        .run();
}
