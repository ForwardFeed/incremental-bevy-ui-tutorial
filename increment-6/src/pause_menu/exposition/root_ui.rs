use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use super::{align_items_ui::spawn_all_align_items, justify_text_ui::spawn_text_and_border_exposition};

#[derive(Component)]
pub struct PauseMenuExpositionUiTag;

pub const COLOR_BG:      Color = Color::srgb(0.20, 0.15, 0.25);
pub const COLOR_BG_A:    Color = Color::srgb(0.25, 0.15, 0.25);
pub const COLOR_BG_B:    Color = Color::srgb(0.20, 0.20, 0.25);
/* const COLOR_OVER:    Color = Color::srgb(0.25, 0.25, 0.25);
const COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
const COLOR_NONE:    Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);
const COLOR_RETURN:  Color = Color::srgb(0.75, 0.35, 0.35); */


pub fn spawn_pause_menu_exposition(
    mut commands: Commands,
){
    commands
    .spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        PauseMenuExpositionUiTag,
        children![ 
            (
                Node {
                    width: Val::Percent(90.),
                    height: Val::Percent(80.),
                    flex_direction: FlexDirection::Row,
                    margin: UiRect {
                        left: Val::Percent(2.),
                        right: Val::Percent(8.),
                        top: Val::Percent(10.),
                        bottom: Val::Percent(10.)
                    },
                    ..Default::default()
                },
                BackgroundColor(COLOR_BG),
                Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>|{
                    spawn_sidebar(parent);
                    spawn_main_content_holder(parent);
                    /* spawn_text_and_border_exposition();
                    spawn_all_align_items(); */
                }))
            ),
            
        ]
    ));
}


fn spawn_main_content_holder(parent: &mut RelatedSpawner<ChildOf>){
    parent.spawn(
        (
            Node{
                width: Val::Percent(80.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            BackgroundColor(COLOR_BG_A)
        )
    );
}


pub fn spawn_sidebar(parent: &mut RelatedSpawner<ChildOf>){
    parent.spawn(
        (
            Node{
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(COLOR_BG_B),
            Children::spawn(SpawnWith(spawn_all_buttons_sidebar))
        )
    )
    
    ;
}

pub fn spawn_all_buttons_sidebar(parent: &mut RelatedSpawner<ChildOf>){
    parent.spawn((
        Node{
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            sidebar_buttons_widget("align_items"),
            sidebar_buttons_widget("justify_text"),
        ]
    ));
}

pub fn sidebar_buttons_widget<T: Into<String>>(text: T) -> impl Bundle{
    (
        Node{
            ..Default::default()
        },
        children![
            Node{
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            Text::new(text),
        ]
        
    )
}