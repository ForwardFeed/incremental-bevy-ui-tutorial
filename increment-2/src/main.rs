use bevy::prelude::*;

mod state;
mod actions;
mod key_bindings;
mod pause_menu;
mod camera;

/// This function main will:
/// 
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
