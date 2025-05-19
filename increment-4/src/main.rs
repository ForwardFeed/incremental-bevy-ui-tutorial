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

/// This function main will:
/// Allow to nagivate through the pause menu using WASD and Enter keys.
fn main() {
    App::new()
        .add_plugins(
            (   
                DefaultPlugins, 
                state::StatesPlugin,
                key_bindings::KeybindingsPlugins,
                pause_menu::pause_menu::PauseMenuPlugin,
                camera::CameraPlugin,
            )
        )
        .run();
}
