use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Controls>::default())
            .add_startup_system(spawn_controls);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Controls {
    Up,
    Down,
    Left,
    Right,
}

fn spawn_controls(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<Controls> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::W, Controls::Up),
            (KeyCode::S, Controls::Down),
            (KeyCode::A, Controls::Left),
            (KeyCode::D, Controls::Right),
        ]),
    });
}
