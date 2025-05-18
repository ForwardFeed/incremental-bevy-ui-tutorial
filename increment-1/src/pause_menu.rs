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
                (   // I use a flex wrapper node here for styling purpose
                    Node {
                        // I use percents here, but you can use Px(pixels)
                        // I think most people uses pixels, as percent has some small issues.
                        width: Val::Percent(30.),
                        height: Val::Percent(30.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Default::default()
                    },
                    // I comment this out because it can be attractive to use the macro children!
                    // But if we want to observe clicks, then we need a less convenient way to write it
                    /* children![
                        pause_menu_button_widget("Resume"),
                        pause_menu_button_widget("Settings"),
                        pause_menu_button_widget("Quit"),
                    ] */
                    // There's a multitude of way to spawn with the parents
                    // I'm only using the way that makes it the more convenient for me.
                    // As you can see, this is a LOT of tabulations, which I'll try to answer by splitting
                    // my code into a sub function because one small mistake and the world turns red.
                    Children::spawn(SpawnWith(spawn_pause_menu_root_buttons))
                )
            ]
        ))
    ;
}

fn spawn_pause_menu_root_buttons(parent: &mut RelatedSpawner<ChildOf>){
    parent.spawn(pause_menu_button_widget("Resume"))
        // A widget with a button in it can be observed for some events.
        // Since it's a system, you can query the stuff you want in it.
        // Resume means to change the state of the application in our case
        .observe(|_trigger: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<PauseState>>|{
            next_state.set(PauseState::Game)
    });
    parent.spawn(pause_menu_button_widget("Settings"))
        .observe(|_trigger: Trigger<Pointer<Click>>|{
            info!("Let's see that functionnality in the next increment");
    });
    parent.spawn(pause_menu_button_widget("Quit"))
        // The correct way to exit a bevy application is to use the AppExit Event
        .observe(|_trigger: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>|{
            exit.write(AppExit::Success);
    });
}


/// Returns a Bundle (very handy bevy 0.16 thing) which can be called a widget.
/// the <T: Into<String>> is just a convenient generic to use so I don't have to String::new()
fn pause_menu_button_widget<T: Into<String>>(inner_text: T) -> impl Bundle{
    (
        Node {
            // imitates a flex-grow-1 from CSS
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
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




