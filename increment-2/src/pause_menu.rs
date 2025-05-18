use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::{actions::PauseMenuActions, state::PauseState};

#[derive(Component)]
pub struct PauseMenuUITag;

fn spawn_pause_menu(
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
        .observe(|_trigger: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<PauseState>>|{
            next_state.set(PauseState::Game)
    });
    parent.spawn(pause_menu_button_widget("Settings"))
        .observe(|_trigger: Trigger<Pointer<Click>>|{
            info!("Let's see that functionnality in the next increment");
    });
    parent.spawn(pause_menu_button_widget("Quit"))
        .observe(|_trigger: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>|{
            exit.write(AppExit::Success);
    });
}


fn pause_menu_button_widget<T: Into<String>>(inner_text: T) -> impl Bundle{
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,            
            border: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        // gives a grayish background color
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        Button,
        children![
            (
                Text(inner_text.into()),
            )
        ]
        
    )
}

/// Utility function that will despawn any entity attached with given component
fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

fn controls(
    query_pause_actions: Query<&ActionState<PauseMenuActions>>,
    current_state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>
){
    for action in query_pause_actions{
        if action.just_pressed(&PauseMenuActions::Activate){
            match current_state.get() {
                PauseState::Game => next_state.set(PauseState::PauseMenu),
                PauseState::PauseMenu => next_state.set(PauseState::Game),
            }
        }
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin{
    fn build(&self, app: &mut App) {

        app
            .add_systems(Update, controls)
            .add_systems(OnEnter(PauseState::PauseMenu), spawn_pause_menu)
            .add_systems(OnExit(PauseState::PauseMenu), despawn::<PauseMenuUITag>)
        ;
    }
}




