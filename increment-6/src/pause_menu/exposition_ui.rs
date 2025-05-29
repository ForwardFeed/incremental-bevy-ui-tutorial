use bevy::prelude::*;


const COLOR_BG:      Color = Color::srgb(0.20, 0.15, 0.25);
const COLOR_BG_A:    Color = Color::srgb(0.25, 0.15, 0.25);
//const COLOR_BG_B:    Color = Color::srgb(0.20, 0.20, 0.25);
/* const COLOR_OVER:    Color = Color::srgb(0.25, 0.25, 0.25);
const COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
const COLOR_NONE:    Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);
const COLOR_RETURN:  Color = Color::srgb(0.75, 0.35, 0.35); */


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
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..Default::default()
        },
        PauseMenuExpositionUiTag,
        children![ 
            (
                Node {
                    width: Val::Percent(80.),
                    height: Val::Percent(80.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                BackgroundColor(COLOR_BG),
                children![
                    spawn_text_and_border_exposition()
                ]
            ),
            
        ]
    ));
}

/* fn row_describe_widget<T: Into<String>>(text: T) -> impl Bundle{
    (
        Node {
            width: Val::Px(200.),
            height: Val::Px(200.),
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Text::new(text)
    )
} */


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

fn spawn_text_and_border_exposition() -> impl Bundle{
    (
        // You cannot have a UI node without a node.
        Node {
            ..Default::default()
        },
        BackgroundColor(COLOR_BG_A),
        children![
            border_widget(text_widget("::Justified", JustifyText::Justified)),
            border_widget(text_widget("::Center", JustifyText::Center)),
            // JustifyText::Left is the default, if you want it you can omit it 
            border_widget(Text::new("::Left")),
            border_widget(text_widget("::Right", JustifyText::Right)),
        ]
        
        
    )
}