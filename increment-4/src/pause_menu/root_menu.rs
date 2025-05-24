use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use crate::state::PauseState;

use super::shared_widgets::{hover_observer, out_observer, pause_menu_button_widget, pressed_observer};


#[derive(Component)]
pub struct PauseMenuUITag;

pub fn spawn_pause_menu(
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
            PauseMenuUITag,
            children![ 
                (
                    Node {
                        width: Val::Percent(30.),
                        height: Val::Percent(30.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Default::default()
                    },
                    Children::spawn(SpawnWith(spawn_pause_menu_root_buttons))
                )
            ]
        ))
    ;
}

fn spawn_pause_menu_root_buttons(parent: &mut RelatedSpawner<ChildOf>){
    parent.spawn(pause_menu_button_widget("Resume"))
    .observe(|_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>|{
        next_state.set(PauseState::Game)
    })
    .observe(hover_observer)  
    .observe(out_observer)
    .observe(pressed_observer);
parent.spawn(pause_menu_button_widget("Settings" ))
    .observe(|_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>|{
        next_state.set(PauseState::PauseMenuSettings)
    })
    .observe(hover_observer)  
    .observe(out_observer)
    .observe(pressed_observer);

parent.spawn(pause_menu_button_widget("Quit"))
    .observe(|_trigger: Trigger<Pointer<Released>>, mut exit: EventWriter<AppExit>|{
        exit.write(AppExit::Success);
    })
    .observe(hover_observer)  
    .observe(out_observer)
    .observe(pressed_observer);
}