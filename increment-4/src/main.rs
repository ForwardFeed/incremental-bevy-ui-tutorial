use bevy::prelude::*;
mod state;
mod actions;
mod key_bindings;

mod pause_menu{
    pub mod pause_menu;
    mod shared_widgets;
    mod root_menu;
    mod settings_menu; 
    mod fun_grids;
}
mod camera;
mod directional;
mod fake_input;

/// This function main will:
/// Allow to nagivate through the pause menu using WASD, Enter, and Return keys.
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
