use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use crate::{state::ExpositionState, theme::{COLOR_BG_SOFT, COLOR_NONE, COLOR_OVER, COLOR_PRESSED}};

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
        ($text:tt, $state:ident) => {
            parent
            .spawn(sidebar_buttons_widget($text))
            .observe(hover_in_observer)
            .observe(pressed_observer)
            .observe(hover_out_observer)
            .observe(|_trigger: Trigger<Pointer<Released>>, mut state: ResMut<NextState<ExpositionState>>|{
                state.set(ExpositionState::$state)
            })
        };
    }
    p_spawn!("text_align", AlignItems);
    p_spawn!("justify_text", JustifyText);
    p_spawn!("box_shadow", BoxShadow);
    p_spawn!("grid", Grid);
}

pub fn sidebar_buttons_widget<T: Into<String>>(text: T) -> impl Bundle{
    (
        Node{
            width: Val::Percent(100.),
            border: UiRect{
                left: Val::Px(5.),
                right: Val::Px(0.),
                top: Val::Px(5.),
                bottom: Val::Px(5.),
            },
            margin: UiRect::top(Val::Px(2.)),
            overflow: Overflow::hidden(),
            ..Default::default()
        },
        BackgroundColor(COLOR_BG_SOFT),
        BorderColor(Color::BLACK),
        BorderRadius{
            top_left: Val::Px(f32::MAX),
            top_right: Val::Px(0.),
            bottom_left: Val::Px(f32::MAX),
            bottom_right: Val::Px(0.),
        },
        ExpositionSidebarButtonTag,
        children![
            (
                Node{
                    margin: UiRect::all(Val::Px(5.)),
                    ..Default::default()
                },
                Pickable::IGNORE,
                TextLayout::new_with_justify(JustifyText::Center),
                Text::new(text),
            )
        ]
        
    )
}


fn hover_in_observer(trigger: Trigger<Pointer<Over>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<ExpositionSidebarButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_OVER.into();
        }  
    }
}

fn hover_out_observer(trigger: Trigger<Pointer<Out>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<ExpositionSidebarButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_NONE.into();
        }  
    }
}

fn pressed_observer(trigger: Trigger<Pointer<Pressed>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<ExpositionSidebarButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_PRESSED.into();
        }  
    }
}
