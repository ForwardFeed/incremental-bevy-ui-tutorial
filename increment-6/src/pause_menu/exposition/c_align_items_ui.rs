use bevy::prelude::*;

use crate::theme::{COLOR_BG, COLOR_BG_SOFT};


pub fn align_items_ui() -> impl Bundle {
    (
        Node{
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(COLOR_BG_SOFT),
        children![
            align_items_widget("AlignItems::Default", AlignItems::Default),
            align_items_widget("AlignItems::Start", AlignItems::Start),
            align_items_widget("AlignItems::End", AlignItems::End),
            align_items_widget("AlignItems::FlexStart", AlignItems::FlexStart),
            align_items_widget("AlignItems::FlexEnd", AlignItems::FlexEnd),
            align_items_widget("AlignItems::Center", AlignItems::Center),
            align_items_widget("AlignItems::Baseline", AlignItems::Baseline),
            align_items_widget("AlignItems::Stretch", AlignItems::Stretch),
        ]
    )
}

fn align_items_widget<T: Into<String>>(text: T, align: AlignItems)-> impl Bundle{
    (
        Node{
            flex_direction: FlexDirection::Column,
            align_items: align,
            flex_grow: 1.0,
            border: UiRect::all(Val::Percent(0.5)),
            ..Default::default()
        },
        BorderColor(Color::BLACK),
        BackgroundColor(COLOR_BG),
        children![
            (
                Node{
                    ..Default::default()
                },
                Text::new(text)
            ),
            (
                Node{
                    ..Default::default()
                },
                Text::new("2n item")
            )
        ]
    )
}
