use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, input_focus::{directional_navigation::DirectionalNavigationMap, InputFocus}, math::CompassOctant, prelude::*};

use crate::state::PauseState;

use super::shared_widgets::{hover_observer, out_observer, pause_menu_button_widget, pressed_observer};


#[derive(Component)]
pub struct PauseMenuUITag;

pub fn spawn_pause_menu(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>
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
                    Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>|{
                        directional_nav_map.add_edges(&spawn_pause_menu_root_buttons(parent), CompassOctant::South);
                    }))
                )
            ]
        )
    );

    // iterate and add edges

}

fn spawn_pause_menu_root_buttons(parent: &mut RelatedSpawner<ChildOf>) -> Vec<Entity>{
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
    return vec![];
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