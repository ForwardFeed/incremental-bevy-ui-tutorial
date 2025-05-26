use bevy::prelude::*;

mod state;
mod actions;
mod key_bindings;

mod pause_menu{
    pub mod pause_menu;
    mod shared_widgets;
    mod root_menu;
    mod settings_menu; 
}
mod camera;
// three new files
mod directional;
mod fake_input;
mod focus;

/// This function main will:
/// Nagivate through the pause menu using WASD and Enter keys.
fn main() {
    App::new()
        .add_plugins(
            (   
                DefaultPlugins, 
                state::StatesPlugin,
                key_bindings::KeybindingsPlugins,
                pause_menu::pause_menu::PauseMenuPlugin,
                camera::CameraPlugin,
                // new plugins
                directional::DirectionalPlugin,
                focus::FocusPlugin,
            )
        )
        .run();
}
