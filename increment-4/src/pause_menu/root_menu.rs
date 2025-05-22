use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use crate::{directional::DirectionalNavigator, dirq, state::PauseState};

use super::shared_widgets::{hover_observer, out_observer, pause_menu_button_widget, pressed_observer};

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum RootButtons{
    Resume,
    Settings,
    Quit
}

#[derive(Component)]
pub struct PauseMenuUITag;

pub fn spawn_pause_menu(
    mut commands: Commands,
){

    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..Default::default()
        },
        PauseMenuUITag,
        DirectionalNavigator::new(
            [
                [
                    (RootButtons::Resume, dirq!(South))
                ],
                [
                    (RootButtons::Settings, dirq!(South, North))
                ],
                [
                    (RootButtons::Quit, dirq!(North))
                ]
            ]
        ),
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
    ));
    
    
}

fn spawn_pause_menu_root_buttons(parent: &mut RelatedSpawner<ChildOf>){
    
    parent.spawn(pause_menu_button_widget("Resume", RootButtons::Resume))
        .observe(|_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>|{
            next_state.set(PauseState::Game)
        })
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);
    parent.spawn(pause_menu_button_widget("Settings" ,RootButtons::Settings))
        .observe(|_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>|{
            next_state.set(PauseState::PauseMenuSettings)
        })
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);

    parent.spawn(pause_menu_button_widget("Quit", RootButtons::Quit))
        .observe(|_trigger: Trigger<Pointer<Released>>, mut exit: EventWriter<AppExit>|{
            exit.write(AppExit::Success);
        })
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);
    
}