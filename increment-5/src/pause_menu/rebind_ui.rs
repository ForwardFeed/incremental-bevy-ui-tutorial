use bevy::{ecs::{relationship::RelatedSpawner}, prelude::*};
use leafwing_input_manager::{clashing_inputs::BasicInputs, prelude::*};

use crate::{actions::GeneralActions, directional::SpawnWithSouthEdges, focus::{FocusIn, FocusOut}, state::RebindGeneralActionState};

const COLOR_BG:  Color = Color::srgb(0.20, 0.15, 0.25);
/* const COLOR_NORMAL:  Color = Color::srgb(0.15, 0.15, 0.15);
const COLOR_SHADOW:  Color = Color::srgb(0.08, 0.08, 0.08); */
const COLOR_OVER:    Color = Color::srgb(0.25, 0.25, 0.25);
const COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct PauseMenuRebindsUITag;

#[derive(Component)]
pub struct RebindRowTag;

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
            PauseMenuRebindsUITag,
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
                .observe(focus_in)
                .observe(focus_out)
                .observe(released)
                .observe(pressed)
                .observe(|_trigger: Trigger<Pointer<Released>>,
                text_query: Query<(&GeneralActions, &mut Text), With<GeneralActions>>,
                current_state: Res<State<RebindGeneralActionState>>,
                mut next_state: ResMut<NextState<RebindGeneralActionState>>|{
                    match current_state.get(){
                        RebindGeneralActionState::None => {},
                        RebindGeneralActionState::Rebinding(_) => return
                    }
                    for (comp_action, mut text) in text_query{
                        if *comp_action == $action{
                            **text = "Enter new key".into()
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
    ]
}

fn rebind_row_widget<T: Into<String>, U: Component>(
    name: T, 
    keybind_text: String,
    compo_tag: U
) -> impl Bundle{

    (
        Node {
            width: Val::Percent(100.),
            justify_content: JustifyContent::SpaceEvenly,
            ..Default::default()
        },
        RebindRowTag,
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
                compo_tag,
                Text::new(keybind_text)
            )
        ]
        
    )
}


pub fn hover_in(trigger: Trigger<Pointer<Over>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<RebindRowTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_OVER.into();
        }  
    }
}

pub fn hover_out(trigger: Trigger<Pointer<Out>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<RebindRowTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = BackgroundColor(Color::NONE);
        }  
    }
}

pub fn focus_in(trigger: Trigger<FocusIn>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<RebindRowTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target() == entity{
            *color = COLOR_OVER.into();
        }  
    }
}

pub fn focus_out(trigger: Trigger<FocusOut>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<RebindRowTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target() == entity{
            *color = BackgroundColor(Color::NONE);
        }  
    }
}

pub fn pressed(trigger: Trigger<Pointer<Pressed>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<RebindRowTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_PRESSED.into();
        }  
    }
}

pub fn released(trigger: Trigger<Pointer<Released>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<RebindRowTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_OVER.into();
        }  
    }
}



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
    let keycodes = keyboard.get_just_pressed().map(|x|*x).collect::<Vec<KeyCode>>();
    if keycodes.len() == 0{
        return;
    }
    let action = match current_state.get() {
        RebindGeneralActionState::None => return,
        RebindGeneralActionState::Rebinding(general_actions) => general_actions,
    };
    input_map.clear_action(action);
    if keycodes.len() == 1{
        // Simple
        input_map.insert(*action, *keycodes.get(0).unwrap());
    } else {
        // Choords
        input_map.insert(*action, ButtonlikeChord::new(keycodes));
    }
    for (q_action, mut text) in q_text{
        if action == q_action{
            **text = convert_keybind_to_text(input_map.get(action))
        }
    }
    next_state.set(RebindGeneralActionState::None);
}


pub struct RebindPlugin;

impl Plugin for RebindPlugin{
    fn build(&self, app: &mut App) {
        // Start listening to keybinds only if we're not in 
        // The negation is a bit odd, but I couldn't figure out the other way
        // in theory I could find a solution using std::mem::discriminant
        // But that sounds like a whole new can of worms
        app
            .add_systems(Update, listen_to_keyboard_new_key.run_if(
            not(in_state(RebindGeneralActionState::None))))
        ;
    }
}