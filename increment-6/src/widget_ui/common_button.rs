use bevy::prelude::*;

#[derive(Component)]
pub struct CommonButtonTag;

const COLOR_NORMAL:  Color = Color::srgb(0.15, 0.15, 0.15);
const COLOR_SHADOW:  Color = Color::srgb(0.08, 0.08, 0.08);
const COLOR_OVER:    Color = Color::srgb(0.25, 0.25, 0.25);
const COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn common_button_widgets<T: Into<String>>(inner_text: T) -> impl Bundle{
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
        CommonButtonTag,
        children![
            (
                Text(inner_text.into()),
                Pickable::IGNORE
            )
        ]
        
    )
}


pub fn hover_observer(trigger: Trigger<Pointer<Over>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<CommonButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_OVER.into();
        }  
    }
}

pub fn out_observer(trigger: Trigger<Pointer<Out>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<CommonButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_NORMAL.into();
        }  
    }
}

pub fn pressed_observer(trigger: Trigger<Pointer<Pressed>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<CommonButtonTag>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_PRESSED.into();
        }  
    }
}

#[macro_export]
macro_rules! common_button {
    ($parent:ident, $text:tt, $onclick:ident) => {
        $parent.spawn($crate::widget_ui::common_button::common_button_widgets($text))
            .observe($onclick)
            .observe($crate::widget_ui::common_button::hover_observer)  
            .observe($crate::widget_ui::common_button::out_observer)
            .observe($crate::widget_ui::common_button::pressed_observer)
    };
}

#[macro_export]
macro_rules! fn_vertical_row_common_buttons {
    ($fn_name:ident, [$(($text:tt, $onclick:ident)),*]) => {
        fn $fn_name(parent: &mut RelatedSpawner<ChildOf>) -> Vec<Entity>{
            vec![
                $(
                    parent.spawn($crate::widget_ui::common_button::common_button_widgets($text))
                        .observe($onclick)
                        .observe($crate::widget_ui::common_button::hover_observer)  
                        .observe($crate::widget_ui::common_button::out_observer)
                        .observe($crate::widget_ui::common_button::pressed_observer)
                        .id()
                ),*
            ]
        }
    };
}
