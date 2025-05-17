use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

use crate::actions::PauseMenuActions;

fn setup(mut commands: Commands){
    // This will tell the program that when I press the keycode Escape on my keyboard.
    // It will translate it to the activation of the pause menu.
    let input_map = InputMap::new([
        (PauseMenuActions::Activate, KeyCode::Escape)
    ]);
    commands.spawn(input_map);
}

pub struct KeybindingsPlugins;

impl Plugin for KeybindingsPlugins{
    fn build(&self, app: &mut App) {
        app
            // If you forget that, the app inputs will silenciously fail
            .add_plugins(InputManagerPlugin::<PauseMenuActions>::default())
            // This function setup will be run very early and once.
            // As you can see how it is scheduled as startup
            .add_systems(Startup, setup);
            

    }
}