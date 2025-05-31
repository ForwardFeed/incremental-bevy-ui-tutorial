use bevy::prelude::*;

use super::root_ui::{COLOR_BG_A, COLOR_BG_B};

fn align_items_widget<T: Into<String>>(text: T, align: AlignItems)-> impl Bundle{
    (
        Node{
            align_items: align,
            width: Val::Percent(100.),
            height: Val::Px(200.),
            border: UiRect::all(Val::Percent(0.5)),
            ..Default::default()
        },
        BorderColor(Color::BLACK),
        BackgroundColor(COLOR_BG_A),
        children![
            (
                Node{
                    ..Default::default()
                },
                Text::new(text)
            ),
        ]
    )
}

pub fn spawn_all_align_items()-> impl Bundle{
    (
        Node{
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(COLOR_BG_B),
        children![
            align_items_widget("AlignItems::Baseline", AlignItems::Baseline),
            align_items_widget("AlignItems::Center", AlignItems::Center),
            align_items_widget("AlignItems::Start", AlignItems::Start),
            align_items_widget("AlignItems::End", AlignItems::End),
            align_items_widget("AlignItems::FlexStart", AlignItems::FlexStart),
            align_items_widget("AlignItems::FlexEnd", AlignItems::FlexEnd),
            align_items_widget("AlignItems::Stretch", AlignItems::Stretch),
            align_items_widget("AlignItems::Default", AlignItems::Default),
        ]
    )
}