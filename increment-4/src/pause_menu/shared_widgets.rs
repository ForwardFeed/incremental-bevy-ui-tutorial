use bevy::prelude::*;

use crate::focus::{FocusIn, FocusOut};

#[derive(Component)]
pub struct MenuButtonTag;

const COLOR_NORMAL:  Color = Color::srgb(0.15, 0.15, 0.15);
const COLOR_SHADOW:  Color = Color::srgb(0.08, 0.08, 0.08);
const COLOR_OVER:    Color = Color::srgb(0.25, 0.25, 0.25);
const COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn pause_menu_button_widget<T: Into<String>>(inner_text: T) -> impl Bundle{
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,           
            margin: UiRect::top(Val::Px(10.)), 
            border: UiRect::all(Val::Px(5.0)),
            
            ..Default::default()
        },
        BoxShadow::new(
            COLOR_SHADOW,
            Val::Percent(1.), 
            Val::Percent(5.),
            Val::Percent(5.), 
            Val::Px(1.) 
        ),
        BackgroundColor(COLOR_NORMAL),
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        Button,
        MenuButtonTag,
        children![
            (
                Text(inner_text.into()),
                Pickable::IGNORE
            )
        ]
        
    )
}


pub fn hover_observer(trigger: Trigger<Pointer<Over>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<MenuButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_OVER.into();
        }  
    }
}

pub fn out_observer(trigger: Trigger<Pointer<Out>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<MenuButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_NORMAL.into();
        }  
    }
}

pub fn pressed_observer(trigger: Trigger<Pointer<Pressed>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<MenuButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_PRESSED.into();
        }  
    }
}

pub fn focus_in_observer(trigger: Trigger<FocusIn>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<MenuButtonTag>>){
    let target = trigger.target();
    for (entity, mut color) in q_menu_buttons{
        if target == entity{
            *color = COLOR_OVER.into();
        }  
    }
}

pub fn focus_out_observer(trigger: Trigger<FocusOut>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<MenuButtonTag>>){
    let target = trigger.target();
    for (entity, mut color) in q_menu_buttons{
        if target == entity{
            *color = COLOR_NORMAL.into();
        }  
    }
}

// I had some fun writing this macro
// Creates the whole function to generate directionnable buttons
#[macro_export]
macro_rules! fn_vertical_row {
    ($fn_name:ident, [$(($text:tt, $onclick:ident)),*]) => {
        fn $fn_name(parent: &mut RelatedSpawner<ChildOf>) -> Vec<Entity>{
            vec![
                $(
                    parent.spawn(pause_menu_button_widget($text))
                        .observe($onclick)
                        .observe(hover_observer)  
                        .observe(out_observer)
                        .observe(pressed_observer)
                        .observe(super::shared_widgets::focus_in_observer)
                        .observe(super::shared_widgets::focus_out_observer)
                        .id()
                ),*
            ]
        }
    };
}

