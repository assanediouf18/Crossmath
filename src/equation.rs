use std::cmp::Ordering;

use rand::Rng;
use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Operation {
    Plus,
    Minus
}

// x +/- y = result
pub struct Equation {
    x: u32,
    y: u32,
    operator: Operation,
    result: u32,
}

impl Equation {
    pub fn new(x: u32, y: u32, operator: Operation) -> Self {
        let result = get_result(x, y, &operator);
        Self {
            x, y, operator,
            result
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..MAX_NUMBER);
        let y = rng.gen_range(1..MAX_NUMBER);
        let operator = get_random_operator(x, y);
        let result = get_result(x, y, &operator);
        Self {
            x, y,
            operator,
            result
        }
    }

    pub fn random_with(value: u32) -> Self {
        let result: u32;
        let x: u32;
        let y: u32;
        let operator: Operation;

        let mut rng = rand::thread_rng();
        let proba: f64 = rng.gen();
        if proba < 0.33 {
            result = value;
            x = rng.gen_range(1..MAX_NUMBER);
            match x.cmp(&result) {
                Ordering::Greater => {
                    y = x - result;
                    operator = Operation::Minus;
                },
                _ => {
                    y = result - x;
                    operator = Operation::Plus;
                }
            }
        } else if proba < 0.66 {
            x = value;
            y = rng.gen_range(1..MAX_NUMBER);
            operator = get_random_operator(x, y);
            result = get_result(x, y, &operator);
        } else {
            y = value;
            x = rng.gen_range(1..MAX_NUMBER);
            operator = get_random_operator(x, y);
            result = get_result(x, y, &operator);
        }

        Self {
            x, y,
            operator,
            result
        }
    }

    pub fn get_random_number(&self) -> u32 {
        let mut rng = rand::thread_rng();
        let proba: f64 = rng.gen();
        if proba < 0.33 {
            self.result
        } else if proba < 0.66 {
            self.x
        } else {
            self.y
        }
    }

    pub fn get_x(&self) -> u32 {
        self.x
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }

    pub fn get_operation(&self) -> Operation {
        self.operator
    }

    pub fn to_string(&self) -> String {
        let sign = match self.operator {
            Operation::Minus => '-',
            Operation::Plus => '+',
        };
        format!("{} {} {} = {}", self.x, sign, self.y, self.result)
    }

    pub fn show(&self) {
        println!("{}", self.to_string());
    }
}

fn get_random_operator(x: u32, y: u32) -> Operation {
    if y > x {
        Operation::Plus
    } else if rand::thread_rng().gen_range(0..100) > 50 {
        Operation::Minus
    } else {
        Operation::Plus
    }
}

fn get_result(x: u32, y: u32, operator: &Operation) -> u32 {
    match operator {
        Operation::Plus => x + y,
        Operation::Minus => x - y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_formatting() {
        let eq = Equation::new(2, 3, Operation::Plus);
        assert_eq!(eq.to_string(), "2 + 3 = 5", "Addition has failed");

        let eq2 = Equation::new(5, 3, Operation::Minus);
        assert_eq!(eq2.to_string(), "5 - 3 = 2", "Substraction has failed");
    }

    #[test]
    fn check_random_with() {
        let random_match = 2;
        let eq = Equation::random_with(random_match);
        assert!(eq.to_string().contains('2'));
    }
}