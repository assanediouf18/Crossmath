mod equation;
mod utils;
mod insertion;

pub mod prelude {
    pub use crate::equation::*;
    pub const MAX_NUMBER: u32 = 100;
    pub use crate::utils::*;
    pub use std::fmt;
    pub use wasm_bindgen::prelude::*;
    pub use crate::insertion::*;
    pub use rand::Rng;
}

use prelude::*;
use rand::Rng;

extern crate web_sys;

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
                write!(f, "{};", symbol)?;
            }
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

        let mut numbers_positions: Vec<(usize, Direction)> = vec![];
        let mut every_number: Vec<usize> = vec![];

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
            let chosen_position = numbers_positions[idx].0;
            let current_dir = numbers_positions[idx].1;
            let base_nb = grid[chosen_position].clone();
            println!("{}", base_nb);
            let base_nb = base_nb
                .trim()
                .parse()
                .expect(
                    "Grid creation : the string {base_nb} cannot be converted to a u32."
                );

            let param = get_random_match_parameter();
            //Find equation
            let eq = Equation::generate(param, base_nb);

            if let Some(dir) = is_insertion_possible(self.width as i32, &grid, chosen_position, current_dir, param, &eq) {
                let start_position = eq.get_start_position(self.width as i32, dir, param, chosen_position);

                //Insert
                insert_equation(self.width as i32, &mut grid, start_position, dir, eq, &mut numbers_positions);

                //Update equation count
                nb_equations += 1;

                every_number = numbers_positions.iter()
                                .map(|x| x.0)
                                .collect();
                // The element used can't be reached again to link new equations
                numbers_positions = numbers_positions
                                        .iter()
                                        .map(|x| (x.0, x.1))
                                        .filter(|x| x.0 != chosen_position)
                                        .collect();
            }
        }

        self.grid = grid;
        self.crop_grid(every_number);
    }

    fn crop_grid(&mut self, numbers_positions: Vec<usize>) {
        let mut xmin = self.width;
        let mut xmax = 0;
        let mut ymin = self.height;
        let mut ymax = 0;

        for position in numbers_positions {
            let position = position as u32;
            let (x, y) = (position % self.width, position / self.width);
            if x < xmin {
                xmin = x;
            }
            if x > xmax {
                xmax = x
            }
            if y < ymin {
                ymin = y;
            }
            if y > ymax {
                ymax = y
            }
        }

        let new_width = (xmax - xmin) + 1;
        let new_height = (ymax - ymin) + 1;

        let mut grid: Vec<String> = (0..new_width * new_height)
            .map(|_| " ".to_string())
            .collect();

        for x in xmin..=xmax {
            for y in ymin..=ymax {
                let new_idx = ((x - xmin) + ((y - ymin) * new_width)) as usize;
                let idx = self.get_idx(x, y);
                grid[new_idx] = self.grid[idx].clone();
            }
        }

        self.grid = grid;
        self.width = new_width;
        self.height = new_height;
    }

    pub fn get_idx(&self, x: u32, y: u32) -> usize {
        (x + (y * self.width)) as usize
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn generation_safety() {
        Crossmath::new(0); 
    }

    #[test]
    fn generation_works() {
        let entity = Crossmath::new(2);
        entity.render();
    }
}
