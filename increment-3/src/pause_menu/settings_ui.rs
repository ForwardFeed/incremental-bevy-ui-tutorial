use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use crate::state::PauseState;

use super::shared_widgets::{hover_observer, out_observer, pause_menu_button_widget, pressed_observer};

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
    parent.spawn(pause_menu_button_widget("Keybinds"))
        .observe(onclick_keybinds)
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);
    parent.spawn(pause_menu_button_widget("PlaceHolder"))
        .observe(onclick_placeholder)
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);
    parent.spawn(pause_menu_button_widget("Return"))
        .observe(onclick_return)
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);
}


fn onclick_keybinds(_trigger: Trigger<Pointer<Released>>){
    info!("Let's see that functionnality in a future increment.");
}


fn onclick_placeholder(_trigger: Trigger<Pointer<Released>>){
    info!("I haven't set anything about that yet, but who knows, I may need it.");
}



fn onclick_return(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::PauseMenu);
}