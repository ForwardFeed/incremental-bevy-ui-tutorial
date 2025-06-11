use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use super::{main_content_ui::spawn_main_content_holder, sidebar_ui::spawn_sidebar};

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
                Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>|{
                    spawn_sidebar(parent);
                    spawn_main_content_holder(parent);
                }))
            ),
            
        ]
    ));
}


