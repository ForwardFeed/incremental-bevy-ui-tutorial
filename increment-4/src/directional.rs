use std::fmt::Debug;

use bevy::{math::{bool, CompassOctant, CompassQuadrant}, prelude::*};
use leafwing_input_manager::prelude::ActionState;

use crate::actions::GeneralActions;

#[derive(Debug, Clone, Copy)]
pub struct Directions{
    north: bool,
    south: bool,
    east:  bool,
    west:  bool
}

// dirq for Direction Quadrant
#[macro_export]
macro_rules! dirq {
    () => {
        compile_error!("No input provided!");
    };
    ($( $x:ident ),*) => {
        $crate::directional::Directions::new(&[$(
            bevy::math::CompassQuadrant::$x,
        )*])
    };
}

impl Directions{
    pub fn all()-> Self{
        dirq!(South, North, West, East)
    }
    pub fn north_south()-> Self{
        dirq!(South, North)
    }
    pub fn east_west()-> Self{
        dirq!(West, East)
    }
    pub fn new(x: &[CompassQuadrant]) -> Self{
        x.iter().fold(Directions{ north: false, south: false, east: false, west: false }, |mut acc, item|{
            match item {
                CompassQuadrant::North => acc.north = true,
                CompassQuadrant::South => acc.south = true,
                CompassQuadrant::East => acc.east = true,
                CompassQuadrant::West => acc.west = true,
            };
            acc
        })
        
    }
}

#[derive(Component)]
pub struct DirectionalNavigator<A: Component>{
    target_id: usize, //must be split
    
    rows: Vec<Vec<(A, Directions)>>
}

impl<A: Component + Clone> DirectionalNavigator<A>{
    pub fn new<I: IntoIterator<Item = (A, Directions)>>(test: impl IntoIterator<Item = I>) -> Self{
        let rows = test.into_iter().fold(vec![], |mut acc, item|{
            let row = item.into_iter().fold(vec![], |mut acc, item|{
                acc.push(item);
                acc
            });
            acc.push(row);
            acc
        });
        DirectionalNavigator { target_id: 0, rows}
    }

    fn find(&self) -> std::result::Result<(A, Directions, usize, usize, usize, usize), ()> {
        let mut offset = self.target_id;
        let mut n_row = 0;
        let n_max_row = self.rows.len();
        for (_row_id, row) in self.rows.iter().enumerate(){
            // small optimisation
            if row.len() - 1 > offset{
                offset -= row.len() - 1;
            } else {
                let mut n_col = 0;
                for (block_id, block) in row.iter().enumerate(){
                    if block_id == offset{
                        let block_clone = (*block).clone();
                        let n_max_col = row.len();
                        return Ok((block_clone.0, block_clone.1, n_row, n_max_row, n_col, n_max_col).clone());
                    }
                    offset -= 1;
                    n_col += 1;
                }
            }
            n_row += 1;
        }
        Err(())
    }
    fn find_id(&self, n_row: usize, n_col: usize,) -> usize{
        let mut id = 0;
        for (row_id, row_data) in self.rows.iter().enumerate(){
            if n_row != row_id{
                id += row_data.len();
                continue
            }  
            let n_max_col = row_data.len();
            if n_col >= n_max_col{
                id += n_max_col
            } else {
                id += n_col;
                break
            }

        }
        id
    }
    fn navigate(&mut self, direction: CompassQuadrant) -> &mut Self {
        let (_comp, dir, n_row, n_max_row, n_col, n_max_col) = match self.find(){
            Ok(x) => {
                x
            },
            Err(_) => {
                error!("The navigation went out of bounds, please fix it, reseting to 0");
                self.target_id = 0;
                return self;
            },
        };
        
        macro_rules! target {
            ($row:expr, $col:expr) => {
                self.target_id = self.find_id($row, $col)
            };
        }

        match direction{
            CompassQuadrant::North => {
                if dir.north{
                    target!( if n_row == 0 { n_max_row } else { n_row - 1},n_col)
                }
            },
            CompassQuadrant::South => {
                if dir.south{
                    target!( if n_row == n_max_row { 0 } else { n_row + 1},n_col)
                }
            },
            CompassQuadrant::East => {
                if dir.east{
                    target!( n_row , if n_col == n_max_col { 0 } else { n_col + 1})
                }
            },
           
            CompassQuadrant::West => {
                if dir.west{
                    target!( n_row , if n_col == 0 { n_max_col } else { n_col - 1})
                }
            },
        }
        self
    }
}



pub fn navigation<T: Component + Debug + Clone>(
    pause_actions: Single<&ActionState<GeneralActions>>,
    mut navigator: Single<&mut DirectionalNavigator<T>>,
    targets: Query<&T>
){
    // If the user is pressing both left and right, or up and down,
    // it should not move in either direction.
    let net_east_west = pause_actions
        .just_pressed(&GeneralActions::MoveRight) as i8
        - pause_actions
        .just_pressed(&GeneralActions::MoveLeft) as i8;

    let net_north_south = pause_actions
        .just_pressed(&GeneralActions::MoveUp) as i8
        - pause_actions
        .just_pressed(&GeneralActions::MoveDown) as i8;
    
    // Compute the direction that the user is trying to navigate in
    // I'll use a CompassOctan later.
    let maybe_direction = match (net_east_west, net_north_south) {
        (0, 0) => None,
        (0, 1) => Some(CompassQuadrant::North),
        (1, 1) => None,
        (1, 0) => Some(CompassQuadrant::East),
        (1, -1) => None,
        (0, -1) => Some(CompassQuadrant::South),
        (-1, -1) => None,
        (-1, 0) => Some(CompassQuadrant::West),
        (-1, 1) => None,
        _ => None,
    };
    match maybe_direction {
        Some(direction) =>{ 
            info!("{direction:?}");
            navigator.navigate(direction);
        },
        None => {},
    }
    
    /* if let Some(direction) = maybe_direction {
        match directional_navigation.navigate(direction) {
            // In a real game, you would likely want to play a sound or show a visual effect
            // on both successful and unsuccessful navigation attempts
            Ok(entity) => {
                println!("Navigated {direction:?} successfully. {entity} is now focused.");
            }
            Err(e) => println!("Navigation failed: {e}"),
        }
    } */
}