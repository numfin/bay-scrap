use bevy::app::AppExit;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Controls>::default())
            .add_startup_system(spawn_controls)
            .add_system(on_escape);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Controls {
    Up,
    Down,
    Left,
    Right,
    Exit,
}

fn spawn_controls(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<Controls> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::W, Controls::Up),
            (KeyCode::S, Controls::Down),
            (KeyCode::A, Controls::Left),
            (KeyCode::D, Controls::Right),
            (KeyCode::Escape, Controls::Exit),
        ]),
    });
}

fn on_escape(mut exit_events: EventWriter<AppExit>, controls: Query<&ActionState<Controls>>) {
    let controls = controls.single();
    if controls.pressed(Controls::Exit) {
        exit_events.send(AppExit);
    }
}
