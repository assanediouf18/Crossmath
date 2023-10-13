use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Operation {
    Plus,
    Minus,
}

#[derive(Clone, Copy)]
pub enum MatchParameter {
    X,
    Y,
    Result,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        let (result, x, y) = get_result(x, y, &operator);
        Self {
            x,
            y,
            operator,
            result,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..MAX_NUMBER);
        let y = rng.gen_range(1..MAX_NUMBER);
        let operator = get_random_operator();
        let (result, x, y) = get_result(x, y, &operator);
        Self {
            x,
            y,
            operator,
            result,
        }
    }

    pub fn generate(param: MatchParameter, value: u32) -> Self {
        let x: u32;
        let y: u32;
        let result: u32;
        let operator: Operation;

        let mut rng = rand::thread_rng();
        match param {
            MatchParameter::X => {
                x = value;
                y = rng.gen_range(1..MAX_NUMBER);
                operator = if x > y {
                    get_random_operator()
                } else {
                    Operation::Plus
                };
            }
            MatchParameter::Y => {
                y = value;
                x = rng.gen_range(1..MAX_NUMBER);
                operator = get_random_operator();
            }
            MatchParameter::Result => {
                result = value;
                x = rng.gen_range(1..MAX_NUMBER);
                match x.cmp(&result) {
                    Ordering::Greater => {
                        y = x - result;
                        operator = Operation::Minus;
                    }
                    _ => {
                        y = result - x;
                        operator = Operation::Plus;
                    }
                };
                
                return Self {
                    x,
                    y,
                    operator,
                    result,
                };
            }
        }

        let (result, x, y) = get_result(x, y, &operator);

        Self {
            x,
            y,
            operator,
            result,
        }
    }

    pub fn random_with(value: u32) -> Self {
        let result: u32;
        let mut x: u32;
        let mut y: u32;
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
                }
                _ => {
                    y = result - x;
                    operator = Operation::Plus;
                }
            }
        } else if proba < 0.66 {
            x = value;
            y = rng.gen_range(1..MAX_NUMBER);
            operator = get_random_operator();
            (result, x, y) = get_result(x, y, &operator);
        } else {
            y = value;
            x = rng.gen_range(1..MAX_NUMBER);
            operator = get_random_operator();
            (result, x, y) = get_result(x, y, &operator);
        }

        Self {
            x,
            y,
            operator,
            result,
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

    pub fn get_result(&self) -> u32 {
        self.result
    }

    pub fn to_array(&self, dir: Direction) -> Vec<String> {
        let oper = match self.operator {
            Operation::Plus => "+",
            Operation::Minus => "-",
        };
        match (dir, oper) {
            // (Direction::Left, "-") | (Direction::Up, "-") => vec![
            //     self.y.to_string(),
            //     oper.to_string(),
            //     self.x.to_string(),
            //     "=".to_string(),
            //     self.result.to_string(),
            // ],
            _ => vec![
                self.x.to_string(),
                oper.to_string(),
                self.y.to_string(),
                "=".to_string(),
                self.result.to_string(),
            ],
        }
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

fn get_random_operator() -> Operation {
    if rand::thread_rng().gen_range(0..100) > 50 {
        Operation::Plus
    } else {
        Operation::Minus
    }
}

/** Returns (result, x, y) */
fn get_result(x: u32, y: u32, operator: &Operation) -> (u32, u32, u32) {
    match operator {
        Operation::Plus => (x + y, x, y),
        Operation::Minus => match x.cmp(&y) {
            Ordering::Less => (y - x, y, x),
            _ => (x - y, x, y),
        },
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
