use bevy::prelude::*;


const COLOR_BG:      Color = Color::srgb(0.20, 0.15, 0.25);
const COLOR_BG_A:    Color = Color::srgb(0.25, 0.15, 0.25);
const COLOR_BG_B:    Color = Color::srgb(0.20, 0.20, 0.25);
/* const COLOR_OVER:    Color = Color::srgb(0.25, 0.25, 0.25);
const COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
const COLOR_NONE:    Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);
const COLOR_RETURN:  Color = Color::srgb(0.75, 0.35, 0.35); */


#[derive(Component)]
pub struct PauseMenuExpositionUiTag;

pub fn spawn_pause_menu_exposition(
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
        PauseMenuExpositionUiTag,
        children![ 
            (
                Node {
                    width: Val::Percent(80.),
                    height: Val::Percent(80.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                BackgroundColor(COLOR_BG),
                children![
                    //spawn_text_and_border_exposition(),
                    //spawn_all_align_items()
                ]
            ),
            
        ]
    ));
}




