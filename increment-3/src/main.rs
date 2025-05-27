use bevy::prelude::*;

mod state;
mod actions;
mod key_bindings;

mod pause_menu{
    pub mod pause_menu;
    mod shared_widgets;
    mod root_ui;
    mod settings_ui; 
}
mod camera;

/// This function main will:
/// On the pause menu buttons: 
/// Mouse over / mouse click changes background & border color
/// Added Box Shadows
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
