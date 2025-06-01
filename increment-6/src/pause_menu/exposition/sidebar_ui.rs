use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use super::root_ui::{COLOR_BG_B, COLOR_NONE, COLOR_OVER, COLOR_PRESSED};

#[derive(Component)]
pub struct ExpositionSidebarButtonTag;



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
            children![
                (
                    Node{
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    Children::spawn(SpawnWith(spawn_all_buttons_sidebar))
                )
            ]
            
        )
    );
}
pub fn spawn_all_buttons_sidebar(parent: &mut RelatedSpawner<ChildOf>){
    macro_rules! p_spawn {
        ($text:tt) => {
            parent
            .spawn(sidebar_buttons_widget($text))
            .observe(hover_in_observer)
            .observe(pressed_observer)
            .observe(hover_out_observer)
        };
    }
    p_spawn!("text_align");
    p_spawn!("justify_text");
}

pub fn sidebar_buttons_widget<T: Into<String>>(text: T) -> impl Bundle{
    (
        Node{
            width: Val::Percent(100.),
            ..Default::default()
        },
        children![
            (
                Node{
                    ..Default::default()
                },
                ExpositionSidebarButtonTag,
                TextLayout::new_with_justify(JustifyText::Center),
                Text::new(text),
            )
        ]
        
    )
}


pub fn hover_in_observer(trigger: Trigger<Pointer<Over>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<ExpositionSidebarButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_OVER.into();
        }  
    }
}

pub fn hover_out_observer(trigger: Trigger<Pointer<Out>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<ExpositionSidebarButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_NONE.into();
        }  
    }
}

pub fn pressed_observer(trigger: Trigger<Pointer<Pressed>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<ExpositionSidebarButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_PRESSED.into();
        }  
    }
}