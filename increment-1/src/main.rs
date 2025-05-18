use bevy::prelude::*;

mod state;
mod actions;
mod key_bindings;
mod pause_menu;
mod camera;

/// This function main will:
/// Spawn a list of 3 buttons
/// Resume: close the menu.
/// Settings: Will do nothing see in a future increment.
/// Quit: quit the executable.
fn main() {
    App::new()
        .add_plugins(
            (   
                DefaultPlugins, 
                state::StatesPlugin,
                key_bindings::KeybindingsPlugins,
                pause_menu::PauseMenuPlugin,
                camera::CameraPlugin,
            )
        )
        .run();
}
