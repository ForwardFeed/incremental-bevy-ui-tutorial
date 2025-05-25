use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

use crate::actions::PauseMenuActions;

fn setup(mut commands: Commands){
    let input_map = InputMap::new([
        (PauseMenuActions::Activate, KeyCode::Escape)
    ]);
    commands.spawn(input_map);
}

pub struct KeybindingsPlugins;

impl Plugin for KeybindingsPlugins{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PauseMenuActions>::default())
            .add_systems(Startup, setup);
            

    }
}