use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use crate::state::PauseState;

use super::shared_widgets::{hover_observer, out_observer, pause_menu_button_widget, pressed_observer};


#[derive(Component)]
pub struct PauseMenuUIMarker;

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
            PauseMenuUIMarker,
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
    // Nice and tidy
    parent.spawn(pause_menu_button_widget("Resume"))
        .observe(onclick_resume)
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);
    parent.spawn(pause_menu_button_widget("Settings" ))
        .observe(onclick_settings)
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);
    parent.spawn(pause_menu_button_widget("Quit"))
        .observe(onclick_quit)
        .observe(hover_observer)  
        .observe(out_observer)
        .observe(pressed_observer);
}


fn onclick_resume(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::Game)
}

fn onclick_settings(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::PauseMenuSettings)
}

fn onclick_quit(_trigger: Trigger<Pointer<Released>>, mut exit: EventWriter<AppExit>){
    exit.write(AppExit::Success);
}