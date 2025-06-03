use bevy::prelude::*;

use super::root_ui::COLOR_BG_A;

fn border_widget<B: Bundle>(child: B) -> impl Bundle{
    (
        Node {
            width: Val::Percent(100.),
            // to show a border you absolutely need to specify a border
            border: UiRect::all(Val::Percent(0.5)),
            ..Default::default()
        },
        // and border color
        BorderColor(Color::BLACK),
        //BorderRadius::MAX, // Also a radius, if you want to make it more round.
        children![
            // You cannot have a Text node and a border node
            // If you want a text node with a border
            // You need to have the parent with border
            child
        ]
    )
}
// I have added colors too because it's cool.
fn text_widget<T: Into<String>>(text: T, justify_text: JustifyText, color: Color) -> impl Bundle{
    (
        Node {
            ..Default::default()
        },
        Text::new(format!("{}\n2nd line", text.into())),
        TextLayout::new_with_justify(justify_text),
        TextColor(color)
    )
}


pub fn spawn_justify_text() -> impl Bundle{
    (
        Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 1.0,
            ..Default::default()
        },
        BackgroundColor(COLOR_BG_A),
        children![
            // JustifyText::Left is the default
            border_widget(text_widget("JustifyText::Left", JustifyText::Left, Color::srgb(0.5, 0.0, 0.0))),
            border_widget(text_widget("JustifyText::Right", JustifyText::Right, Color::srgb(0.0, 0.5, 0.0))),
            border_widget(text_widget("JustifyText::Justified", JustifyText::Justified, Color::srgb(0.0, 0.0, 0.5))),
            border_widget(text_widget("JustifyText::Center", JustifyText::Center, Color::srgb(0.5, 0.5, 0.0))),
        ]
        
        
    )
}