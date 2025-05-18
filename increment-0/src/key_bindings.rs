use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

use crate::actions::PauseMenuActions;

fn setup(mut commands: Commands){
    // This will tell the program that when I press the keycode Escape on my keyboard.
    // It will translate it to the activation of the pause menu.
    // We could tinker with it and introduce gamepads or support hardware this way.
    let input_map = InputMap::new([
        (PauseMenuActions::Activate, KeyCode::Escape)
    ]);
    commands.spawn(input_map);
}

pub struct KeybindingsPlugins;

impl Plugin for KeybindingsPlugins{
    fn build(&self, app: &mut App) {
        app
            // If you forget that, the inputs & actions aren't effectively binded.
            .add_plugins(InputManagerPlugin::<PauseMenuActions>::default())
            .add_systems(Startup, setup);
            

    }
}