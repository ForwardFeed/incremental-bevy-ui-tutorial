use bevy::prelude::*;

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
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                },
                children![
                     
                ]
            ),
            
        ]
    ));
}

fn row_describe_widget<T: Into<String>>(text: T) -> impl Bundle{
    (
        Node {
            width: Val::Px(200.),
            height: Val::Px(200.),
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Text::new(text.into())
    )
}

fn borders_widget() -> impl Bundle{
    (
        Node {
            width: Val::Px(200.),
            height: Val::Px(200.),
            ..Default::default()
        }
    )
}