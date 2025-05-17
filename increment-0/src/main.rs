use bevy::prelude::*;

mod state;
mod actions;
mod key_bindings;
mod pause_menu;
mod camera;
/// What this main function will do:
/// Press Escape and have a TextBox centered saying hello world. 
/// Repress Escape and it will disappear for the screen.
/// And this forever.
fn main() {
    App::new()
        // Side note you can also .add_plugins multiple times.
        // But you can also bundle it in a n long tuple like here.
        // It doesn't really matter.
        .add_plugins(
            (   
                // DefaultPlugins is a collection of plugins that we usually want (create a window automatically for example.)
                DefaultPlugins, 
                state::StatesPlugin,
                key_bindings::KeybindingsPlugins,
                pause_menu::PauseMenuPlugin,
                camera::CameraPlugin,
            )
        )
        .run();
}
