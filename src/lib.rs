mod equation;
mod utils;

pub mod prelude {
    pub use crate::equation::*;
    pub const MAX_NUMBER : u32 = 100;
    pub use wasm_bindgen::prelude::*;
    pub use crate::utils::*;
    pub use std::fmt;
}

use prelude::*;

#[wasm_bindgen]
pub struct Crossmath {
    width: u32,
    height: u32,
    equations: Vec<Equation>
}

impl fmt::Display for Crossmath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, eq) in self.equations.iter().enumerate() {
            let symbol = eq.to_string();
            write!(f, "{i}. {}\n", symbol)?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Crossmath {
    pub fn new(number_of_equations: u32) -> Self {
        set_panic_hook();

        if number_of_equations <= 0 {
            panic!("Can't create less than 1 equation for the game, got {}.", number_of_equations);
        }

        let size = (number_of_equations / 2) * 5;
        let mut eqs: Vec<Equation> = vec![Equation::random()];
        let mut number_match = eqs[0].get_random_number();
        for i in 1..number_of_equations {
            eqs.push(Equation::random_with(number_match));
            number_match = eqs[i as usize].get_random_number();
        }

        Self {
            width: size,
            height: size,
            equations: eqs,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
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
}
