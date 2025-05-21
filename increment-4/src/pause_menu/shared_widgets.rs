use bevy::prelude::*;


// It's import to tag our button we want to stylize
// In order not to modify all the button of our app
#[derive(Component)]
pub struct MenuButtonTag;

const COLOR_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const COLOR_SHADOW: Color = Color::srgb(0.08, 0.08, 0.08);

pub fn pause_menu_button_widget<T: Into<String>>(inner_text: T, compotag: impl Component) -> impl Bundle{
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
        compotag,
        children![
            (
                Text(inner_text.into()),
            )
        ]
        
    )
}

pub fn pause_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut BorderRadius
        ),
        (Changed<Interaction>, With<MenuButtonTag>),
    >,
) {
    for (interaction, mut color, mut border_color, mut border_radius) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                border_color.0 = Color::srgb(0.45, 0.45, 0.45);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
                *border_radius = BorderRadius{
                    top_left: Val::Px(0.),
                    top_right: Val::Px(f32::MAX),
                    bottom_left: Val::Px(f32::MAX),
                    bottom_right: Val::Px(0.),
                };
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = COLOR_NORMAL.into();
                *border_radius = BorderRadius::MAX;
                border_color.0 = Color::BLACK;
            }
        }
    }
}
