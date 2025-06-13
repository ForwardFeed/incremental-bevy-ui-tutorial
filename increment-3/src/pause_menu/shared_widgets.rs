use bevy::prelude::*;

// It's importance is to mark the button we want to stylize
// In order not to modify all the button of our app
#[derive(Component)]
pub struct MenuButtonMarker;


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
            // x_offset, direction: <= negative positive =>
            Val::Percent(1.), 
            // y_offset, direction: ^ negative positive v
            Val::Percent(5.),
            // spread radius
            Val::Percent(5.), 
            // blur radius
            Val::Px(1.) 
        ),
        BackgroundColor(COLOR_NORMAL),
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        Button,
        // Don't forget our marker for the observers below
        MenuButtonMarker,
        children![
            (
                Text(inner_text.into()),
                // Very important thing, by making it not pickable, it means that it won't trigger
                // the observers below. Because yes if you forget it, 
                // entering in a child element will trigger the exit of the parent element.
                Pickable::IGNORE
            )
        ]
        
    )
}


// Observer Hover, when your mouse goes over it
pub fn hover_observer(trigger: Trigger<Pointer<Over>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<MenuButtonMarker>>){
    // Iterate all Button Marker
    for (entity, mut color) in q_menu_buttons{
        // If the entity is the one that triggered the click event
        if trigger.target == entity{
            // this modifies the color
            *color = COLOR_OVER.into();
        }  
    }
}

// Observer Out, when your mouse goes away from it
pub fn out_observer(trigger: Trigger<Pointer<Out>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<MenuButtonMarker>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_NORMAL.into();
        }  
    }
}

// Observer Press, when your mouse puts its primary down
pub fn pressed_observer(trigger: Trigger<Pointer<Pressed>>, q_menu_buttons: Query<(Entity, &mut BackgroundColor), With<MenuButtonMarker>>){
    for (entity, mut color) in q_menu_buttons{
        if trigger.target == entity{
            *color = COLOR_PRESSED.into();
        }  
    }
}


// BEHOLD, UNUSED CODE
// I've decided not to use what's below, even if it's currently shown in examples
// Reason being that I decided that handling everything simply with Events and Observers was better
// I've actually came back from a future increment to change that as it wasn't great mix with interactivity
// Iteracting with observers is easy you just write fake events
// With Changed, I couldn't find a way to do it

/*

/// This is a system meant to run every frame
/// If a button is being clicked, hovered, or got back to normal
/// Its colors and border color will change
/// Its border radius will also change
pub fn pause_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut BorderRadius
        ),
        (Changed<Interaction>, With<MenuButtonMarker>),
    >,
) {
    // Checks every button interaction
    // And act as forementioned
    for (interaction, mut color, mut border_color, mut border_radius) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                border_color.0 = Color::srgb(0.45, 0.45, 0.45);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
                // gives a border radius I like
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
 */