use bevy::{ecs::{relationship::RelatedSpawner}, prelude::*};
use leafwing_input_manager::{clashing_inputs::BasicInputs, prelude::*};

use crate::{actions::GeneralActions, directional::SpawnWithSouthEdges, state::{PauseState, RebindGeneralActionState}};

const COLOR_BG:  Color = Color::srgb(0.20, 0.15, 0.25);
const COLOR_OVER:    Color = Color::srgb(0.25, 0.25, 0.25);
const COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
const COLOR_NONE:    Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);
const COLOR_RETURN:  Color = Color::srgb(0.75, 0.35, 0.35);

#[derive(Component)]
pub struct PauseMenuRebindsUIMarker;

#[derive(Component)]
pub struct RebindRowMarker;

pub fn spawn_pause_menu_keybinds(
    mut commands: Commands,
    actions: Single<&InputMap<GeneralActions>>,
){
    let actions_cloned = actions.clone();
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
            PauseMenuRebindsUIMarker,
            children![ 
                (
                    Node {
                        width: Val::Percent(80.),
                        height: Val::Percent(80.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Default::default()
                    },
                    BackgroundColor(COLOR_BG),
                    Children::spawn(SpawnWithSouthEdges(move |parent: &mut RelatedSpawner<ChildOf>|{
                        spawn_rebind_rows(parent, actions_cloned)
                    }))
                )
            ]
        ))
    ;
}


fn spawn_rebind_rows(parent: &mut RelatedSpawner<ChildOf>, keybinds: InputMap<GeneralActions>) -> Vec<Entity>{
    // apologies for the macro, it's too convenient
    macro_rules! row {
        ($text:tt, $action:expr) => {
            parent.spawn(rebind_row_widget($text, convert_keybind_to_text(keybinds.get(&$action)), $action))
                .observe(hover_in)
                .observe(hover_out)
                .observe(released)
                .observe(pressed)
                .observe(|_trigger: Trigger<Pointer<Released>>,
                text_query: Query<(&GeneralActions, &mut Text), With<GeneralActions>>,
                current_state: Res<State<RebindGeneralActionState>>,
                mut next_state: ResMut<NextState<RebindGeneralActionState>>,
                input_map: Single<&InputMap<GeneralActions>>|{
                    // I check this condition because, you may click on multiple rows
                    // and then you will have a visual bug.
                    let prev_missclick =  match current_state.get(){
                        RebindGeneralActionState::None => None,
                        RebindGeneralActionState::Rebinding(current_rebinding) => {
                            Some(current_rebinding)
                        }
                    };
                    for (comp_action, mut text) in text_query{
                        // Tells the user it has to enter a new key
                        if *comp_action == $action{
                            **text = "Enter new key".into()
                        }
                        // So in case of a previous user missclick, this will turn back the previous clicked
                        // rebind ui button into its normal state.
                        if let Some(prev_missclick) = prev_missclick{
                            if comp_action == prev_missclick{
                                **text = convert_keybind_to_text(input_map.get(prev_missclick))
                            }
                        }
                    }
                    next_state.set(RebindGeneralActionState::Rebinding($action))
                })
                .id()
        };
    }
    vec![
        row!("Move Up", GeneralActions::MoveUp),
        row!("Move Down", GeneralActions::MoveDown),
        row!("Move Left", GeneralActions::MoveLeft),
        row!("Move Right", GeneralActions::MoveRight),
        row!("Accept", GeneralActions::Accept),
        return_button(parent)
    ]
}

fn rebind_row_widget<T: Into<String>, U: Component>(
    name: T, 
    keybind_text: String,
    compo_marker: U
) -> impl Bundle{
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..Default::default()
        },
        RebindRowMarker,
        children![
            (
                Node{
                    width: Val::Percent(50.),
                    ..Default::default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                Text::new(name.into())
            ),
            (
                Node{
                    width: Val::Percent(50.),
                    ..Default::default()
                },
                compo_marker,
                Text::new(keybind_text)
            )
        ]
        
    )
}

// a simple return button so I get back to where I was.
fn return_button(parent: &mut RelatedSpawner<ChildOf>) -> Entity{
    parent.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..Default::default()
        },
        BackgroundColor(COLOR_RETURN.into()),
        children![
            (
                Node{
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                Text::new("Return")
            )
        ]
    ))
    .observe(|_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>|{
        next_state.set(PauseState::PauseMenuSettings)
    })
    .id()
}

// Yeah I macroed that too because it was taking too much space for me
macro_rules! fn_observer {
    ($name:ident, $event_type:ty, $color:expr) => {
        pub fn $name(trigger: Trigger<$event_type>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<RebindRowMarker>>){
            let target = trigger.target();
            for (entity, mut color) in q_menu_buttons{
                if target == entity{
                    *color = $color.into();
                }  
            }
        }
    };
}
fn_observer!(hover_in, Pointer<Over>, COLOR_OVER);
fn_observer!(hover_out, Pointer<Out>, COLOR_NONE);
fn_observer!(pressed, Pointer<Pressed>, COLOR_PRESSED);
fn_observer!(released, Pointer<Released>, COLOR_OVER);

// The way I convert to text is pretty unsure since I use the debug formatting
// I may expect to improve that in the future.
// It's especially weird for non-qwerty users as when you press Z with an azerty it shows W
// Which is confusing, but to solve that one would have to dwelve into winit input handling.
fn convert_keybind_to_text(keybind: Option<Vec<UserInputWrapper>>) -> String{
    match keybind{
        Some(vec_user_input_wrapper) => {
            vec_user_input_wrapper.iter().map(|user_input_wrapper|{
                match user_input_wrapper {
                    UserInputWrapper::Button(buttonlike) => {
                        match buttonlike.decompose() {
                            BasicInputs::None => {
                                format!("None")
                            },
                            BasicInputs::Simple(buttonlike) => {
                                format!("{:?}", buttonlike)
                            },
                            BasicInputs::Composite(buttonlikes) => {
                                buttonlikes.iter().map(|b|{
                                    format!("{:?}", b)
                                }).collect::<Vec<String>>().join("+")
                            },
                            BasicInputs::Chord(buttonlikes) => {
                                buttonlikes.iter().map(|b|{
                                    format!("{:?}", b)
                                }).collect::<Vec<String>>().join("+")
                            },
                        }
                    },
                    UserInputWrapper::Axis(axislike) => {
                        format!("Axis: {:?}", axislike)
                    },
                    UserInputWrapper::DualAxis(dual_axislike) => {
                        format!("Dual Axis: {:?}", dual_axislike)
                    },
                    UserInputWrapper::TripleAxis(triple_axislike) => {
                        format!("Tripple Axis: {:?}", triple_axislike)
                    },
                }
            }).collect::<Vec<String>>().join(", ")
        },
        None => {
            format!("")
        },
    }
}

fn listen_to_keyboard_new_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<RebindGeneralActionState>>,
    mut next_state: ResMut<NextState<RebindGeneralActionState>>,
    mut input_map: Single<&mut InputMap<GeneralActions>>,
    q_text: Query<(&GeneralActions, &mut Text), With<GeneralActions>>

){
    // retrieve everything that has been just pressed in the last frame.
    let keycodes = keyboard.get_just_pressed().map(|x|*x).collect::<Vec<KeyCode>>();
    if keycodes.len() == 0{
        return;
    }
    let action = match current_state.get() {
        RebindGeneralActionState::None => {
            warn!("How did listen_to_keyboard_new_key be callable it that state?????");
            return;
        }, 
        RebindGeneralActionState::Rebinding(general_actions) => general_actions,
    };
    // Remove all previous binds from this action
    input_map.clear_action(action);
    if keycodes.len() == 1{
        // Simple
        input_map.insert(*action, *keycodes.get(0).unwrap());
    } else {
        // Choords
        // One problem is if you want to introduce a choord.
        // You have to hit all keys at the same frame.
        // The solution to that would be to buffer inputs for some frames.
        // Which is doable yet significantly more difficult
        input_map.insert(*action, ButtonlikeChord::new(keycodes));
    }
    // Replacing the text in the UI
    for (q_action, mut text) in q_text{
        if action == q_action{
            **text = convert_keybind_to_text(input_map.get(action))
        }
    }
    next_state.set(RebindGeneralActionState::None);
}

// this is a custom Condition
// So I don't have to write convoluted scheme like
// run_if(not(in_state(RebindGeneralActionState::None))))
// (I don't like to write with negation if I can avoid it)
// + that's a good tool to know about, so I show it for tutorial purpose.
fn on_keybind_listen() -> impl Condition<()> {
    IntoSystem::into_system(|state: Option<Res<State<RebindGeneralActionState>>>| match state {
        Some(state) => matches!(**state, RebindGeneralActionState::Rebinding(_)),
        None => false,
    })
}


pub struct RebindPlugin;

impl Plugin for RebindPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, listen_to_keyboard_new_key.
                run_if(on_keybind_listen()))
        ;
    }
}