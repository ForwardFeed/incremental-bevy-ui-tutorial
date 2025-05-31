use bevy::prelude::*;

use super::root_ui::COLOR_BG_A;

fn text_widget<T: Into<String>>(text: T, justify_text: JustifyText) -> impl Bundle{
    (
        Node {
            // If your text node doesn't take more space
            // The TextLayout Will likely be 
            width: Val::Percent(100.),
            ..Default::default()
        },
        Text::new(text),
        TextLayout::new_with_justify(justify_text)
    )
}

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

pub fn spawn_text_and_border_exposition() -> impl Bundle{
    (
        // You cannot have a UI node without a node.
        Node {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(COLOR_BG_A),
        children![
            border_widget(text_widget("JustifyText::Justified", JustifyText::Justified)),
            border_widget(text_widget("JustifyText::Center", JustifyText::Center)),
            // JustifyText::Left is the default, if you want it you can omit it 
            border_widget(Text::new("JustifyText::Left")),
            border_widget(text_widget("JustifyText::Right", JustifyText::Right)),
        ]
        
        
    )
}