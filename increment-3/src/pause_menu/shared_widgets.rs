use bevy::prelude::*;

pub fn pause_menu_button_widget<T: Into<String>>(inner_text: T) -> impl Bundle{
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,            
            border: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        Button,
        children![
            (
                Text(inner_text.into()),
            )
        ]
        
    )
}