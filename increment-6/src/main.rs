use bevy::prelude::*;

mod state;
mod actions;
mod key_bindings;

mod pause_menu{
    pub mod pause_menu;
    mod shared_widgets;
    mod root_ui;
    mod settings_ui; 
    mod rebind_ui;
    mod exposition_ui;
}
mod camera;
mod directional;
mod fake_input;
mod focus;

/// This function main will:
fn main() {
    App::new()
        .add_plugins(
            (   
                DefaultPlugins, 
                state::StatesPlugin,
                key_bindings::KeybindingsPlugins,
                pause_menu::pause_menu::PauseMenuPlugin,
                camera::CameraPlugin,
                directional::DirectionalPlugin,
                focus::FocusPlugin,
            )
        )
        .run();
}
