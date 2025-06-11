use bevy::{ecs::spawn::SpawnIter, prelude::*};


pub fn grid_ui() -> impl Bundle{
    (
        Node{
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            // Absolutely need to say that we want a grid
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(3, 0.5),
            grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
            row_gap: Val::Px(12.0),
            column_gap: Val::Px(12.0),
            ..Default::default()
        },
        // SpawnIter is a bevy utils useful to spawn with an iterator
        Children::spawn(SpawnIter(
            (0..12).into_iter().map(|index|{
                grid_element_ui(index + 1)
            }),
        ))
    )
}


fn grid_element_ui(index: i32) -> impl Bundle{
    (
        Node{
            border: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        BorderColor(color_from_index(index as f32, true)),
        /* BoxShadow::new(
            color_from_index(index as f32, true),
            Val::Px(3.0),
            Val::Px(3.0),
            Val::Px(6.0),
            Val::Px(1.0),
        ), */
        BackgroundColor(color_from_index(index as f32, false)),
        children![
            Text::new(index.to_string())
        ]
        
    )
}
// d for darker tone
fn color_from_index(n: f32, d: bool) -> Color{
    // darker tone as fload
    let df = if d {-0.2}else{0.0};
    let r = (n * 0.1 % 1.0) - df;
    let g = (n * 0.3 % 1.0) - df;
    let b = (n * 0.6 % 1.0) - df; 
    return Srgba::new(r, g, b, 1.0).into()
}