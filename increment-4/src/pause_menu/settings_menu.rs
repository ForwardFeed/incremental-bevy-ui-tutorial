use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use crate::state::PauseState;

use super::shared_widgets::pause_menu_button_widget;


#[derive(Component, Debug)]
pub enum SettingsButtons{
    Keybinds,
    PlaceHolder,
    Return
}


#[derive(Component)]
pub struct PauseMenuSettingsUITag;

pub fn spawn_pause_menu_settings(
    mut commands: Commands,
){
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            PauseMenuSettingsUITag,
            children![ 
                (
                    Node {
                        width: Val::Percent(30.),
                        height: Val::Percent(30.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Default::default()
                    },
                    Children::spawn(SpawnWith(spawn_pause_menu_settings_buttons))
                )
            ]
        ))
    ;
}



fn spawn_pause_menu_settings_buttons(parent: &mut RelatedSpawner<ChildOf>){
    parent.spawn(pause_menu_button_widget("Keybinds", SettingsButtons::Keybinds))
        .observe(|_trigger: Trigger<Pointer<Click>>|{
            info!("Let's see that functionnality in a future increment.");
    });
    parent.spawn(pause_menu_button_widget("PlaceHolder", SettingsButtons::PlaceHolder))
        .observe(|_trigger: Trigger<Pointer<Click>>|{
            info!("I haven't set anything about that yet, but who knows, I may need it.");
    });
    parent.spawn(pause_menu_button_widget("Return", SettingsButtons::Return))
        .observe(|_trigger: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<PauseState>>|{
            next_state.set(PauseState::PauseMenu);
    });
}
