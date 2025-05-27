use bevy::prelude::*;

mod state;
mod actions;
mod key_bindings;
// I have split the code into multiple file in a submodule
mod pause_menu{
    pub mod pause_menu; // handles the cohesive logic in the submodule
    mod shared_widgets; // Handles the shared code of the submodule (mostly widgets)
    mod root_ui; // handles the logic and structure inherent of the top level UI
    mod settings_ui; // handles the logic and structure inherent of the settings sub menu
}
mod camera;

/// This function main will:
/// Create a submenu for settings
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
