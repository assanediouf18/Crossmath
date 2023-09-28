mod equation;
mod utils;

pub mod prelude {
    pub use crate::equation::*;
    pub const MAX_NUMBER: u32 = 100;
    pub use crate::utils::*;
    pub use std::fmt;
    pub use wasm_bindgen::prelude::*;
}

use prelude::*;
use rand::Rng;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
pub struct Crossmath {
    width: u32,
    height: u32,
    grid: Vec<String>,
}

impl fmt::Display for Crossmath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.get_idx(x, y);
                let symbol = self.grid[i as usize].clone();
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Crossmath {
    pub fn new(number_of_equations: u32) -> Self {
        // set_panic_hook();

        if number_of_equations <= 0 {
            panic!(
                "Can't create less than 1 equation for the game, got {}.",
                number_of_equations
            );
        }

        let size = 2 * 5 * number_of_equations;

        let mut crossmath = Self {
            width: size,
            height: size,
            grid: vec![],
        };

        crossmath.create_grid(number_of_equations);

        crossmath
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn create_grid(&mut self, nb_of_equations: u32) {
        let mut grid: Vec<String> = (0..self.width * self.height)
            .map(|_| " ".to_string())
            .collect();

        let mut numbers_positions: Vec<usize> = vec![];

        //Insert the first equation
        insert_equation(
            self.width as i32,
            &mut grid,
            self.get_idx(self.width / 2, self.height / 2),
            Direction::Right,
            Equation::random(),
            &mut numbers_positions
        );
        let mut nb_equations = 1;

        let mut rng = rand::thread_rng();
        while nb_equations < nb_of_equations {
            //Pick a random number for the next equation
            let idx = rng.gen_range(0..numbers_positions.len());
            let base_nb = grid[numbers_positions[idx]].clone();
            println!("{}", base_nb);
            let base_nb = base_nb
                .trim()
                .parse()
                .expect(
                    "Grid creation : the string at the position {idx} cannot be converted to a u32."
                );

            if let Some((dir, param)) = is_insertion_possible(self.width as i32, &grid, idx) {
                let delta = get_direction_step(dir, self.width as i32);
                let start_position = match param {
                    MatchParameter::Y => (numbers_positions[idx] as i32) - 2 * delta,
                    MatchParameter::Result => (numbers_positions[idx] as i32) - 4 * delta,
                    MatchParameter::X => numbers_positions[idx] as i32
                } as usize;

                //Find equation
                let eq = Equation::generate(param, base_nb);

                //Insert
                insert_equation(self.width as i32, &mut grid, start_position, dir, eq, &mut numbers_positions);

                //Update equation count
                nb_equations += 1;

                //Add position to the list of positions of cells holding numbers
                numbers_positions.push(start_position);
            }
        }

        self.grid = grid;
    }

    pub fn get_idx(&self, x: u32, y: u32) -> usize {
        (x + (y * self.width)) as usize
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

// On va faire simple dans un premier temps :
// - On ne regarde pas s'il est possible de faire une liaison ici avec le résultat
fn is_insertion_possible(
    width: i32,
    grid: &Vec<String>,
    position: usize,
) -> Option<(Direction, MatchParameter)> {
    let directions = [Direction::Left, Direction::Down, Direction::Up, Direction::Right];
    let origin = position;
    for dir in directions {
        let delta = get_direction_step(dir, width);
        for i in (1..=4).rev() {
            let position = origin as i32 + i * delta;

            if position < 0 {
                break;
            }

            let position = position  as usize;
            if position > grid.len() {
                // println!("The position is too big : {} from {} with delta={}", position, origin, delta);
                break;
            }

            if grid[position] != " " {
                if i > 2 {
                    return Some((dir, MatchParameter::Y));
                } else {
                    return None;
                }
            } else if i == 1 {
                return Some((dir, MatchParameter::X));
            }
        }
    }
    None
}

fn insert_equation(width: i32, grid: &mut Vec<String>, start_position: usize, dir: Direction, eq: Equation, marked: &mut Vec<usize>) {
    let delta = get_direction_step(dir, width);

    log!("{}", eq.to_string());
    
    let representation = eq.to_array();
    for (idx, term) in representation.iter().enumerate() {
        let position = (start_position as i32 + (idx as i32 * delta)) as usize;
        if position > grid.len() {
            let direction = match dir {
                Direction::Down => "Down",
                Direction::Up => "Up",
                Direction::Left =>"Left",
                Direction::Right => "Right",
            };
            panic!("Index out of bounds : dir={} delta={} start={} size={}", direction, delta, start_position, width);
        }
        grid[position] = term.to_string();
        match term.trim().parse::<u32>() {
            Ok(_) => marked.push(position),
            Err(_) => {}
        };
    }
}

fn get_direction_step(dir: Direction, width: i32) -> i32 {
    match dir {
        Direction::Down => width,
        Direction::Up => -width,
        Direction::Left => -1,
        Direction::Right => 1
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    #[should_panic]
    fn generation_safety() {
        Crossmath::new(0);
    }

    #[test]
    fn test_is_insertion_possible() {
        let grid: Vec<String> = (0..100).map(|_| " ".to_string()).collect();
        match is_insertion_possible(100, &grid, 50) {
            Some(_) => {},
            None => assert!(false, "The insertion is not working")
        }
    }

    #[test]
    fn test_insertion() {
        let eq = Equation::new(3, 4, Operation::Plus);
        assert_eq!(eq.to_array(), ["3", "+", "4", "=", "7"]);
    }

    #[test]
    fn generation_works() {
        let entity = Crossmath::new(2);
        entity.render();
    }
}
