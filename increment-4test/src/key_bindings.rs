use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

use crate::actions::{GeneralActions, PauseMenuActions};

fn setup(mut commands: Commands){
    let input_map_pause_menu = InputMap::new([
        (PauseMenuActions::Activate, KeyCode::Escape)
    ]);
    commands.spawn(input_map_pause_menu);

    // Generating new keys for the new interactions.
    let input_map_general_actions = InputMap::new([
        (GeneralActions::MoveUp, KeyCode::KeyW),
        (GeneralActions::MoveDown, KeyCode::KeyS),
        (GeneralActions::MoveLeft, KeyCode::KeyA),
        (GeneralActions::MoveRight, KeyCode::KeyD),
        (GeneralActions::Accept, KeyCode::Enter),
        (GeneralActions::Deny, KeyCode::Backspace),
    ]);
    commands.spawn(input_map_general_actions);
}

pub struct KeybindingsPlugins;

impl Plugin for KeybindingsPlugins{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PauseMenuActions>::default())
            .add_plugins(InputManagerPlugin::<GeneralActions>::default())
            .add_systems(Startup, setup);
            

    }
}