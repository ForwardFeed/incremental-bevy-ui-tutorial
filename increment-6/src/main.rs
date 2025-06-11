use bevy::prelude::*;

mod state;
mod actions;
mod key_bindings;

mod pause_menu{
    pub mod pause_menu;
    mod root_ui;
    mod settings_ui; 
    mod rebind_ui;
    pub mod exposition{
        pub mod exposition;
        mod root_ui;
        mod sidebar_ui;
        mod main_content_ui;
        // c_ because it's sorted alphabetically, without needing me to make a subfolder
        mod c_align_items_ui;
        mod c_justify_text_ui;
        mod c_box_shadow_ui;
    }
}
// new mod to put all widgets somewhere
mod widget_ui{
    pub mod common_button;
}
mod camera;
mod directional;
mod fake_input;
mod focus;
mod ecs;
/// This function main will:
/// Show a sub menu called exposition, where I'll expose many stylisation for UI
/// 
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
