use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
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
        let mut x: u32;
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
                if operator == Operation::Minus && x < y {
                    x = rng.gen_range(y..MAX_NUMBER);
                }
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
            (Direction::Left, "-") | (Direction::Up, "-") => vec![
                self.y.to_string(),
                oper.to_string(),
                self.x.to_string(),
                "=".to_string(),
                self.result.to_string(),
            ],
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

    pub fn get_start_position(&self, grid_width: i32, dir: Direction, param: MatchParameter, param_position: usize) -> usize {
        let position = param_position;
        let dir_step = get_direction_step(dir, grid_width);
        let start_position = match param {
            MatchParameter::Y => match (self.operator, dir) {
                (Operation::Minus, Direction::Left) | (Operation::Minus, Direction::Up) => position as i32,
                _ => (position as i32) - 2 * dir_step
            },
            MatchParameter::X => match (self.operator, dir) {
                (Operation::Minus, Direction::Left) | (Operation::Minus, Direction::Up) => position as i32 - 2 * dir_step,
                _ => position as i32
            },
            MatchParameter::Result => (position as i32) - 4 * dir_step,
        } as usize;
        start_position
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

pub fn get_direction_step(dir: Direction, width: i32) -> i32 {
    match dir {
        Direction::Down => width,
        Direction::Up => -width,
        Direction::Left => -1,
        Direction::Right => 1,
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

    #[test]
    fn test_representation() {
        let eq = Equation::new(3, 4, Operation::Plus);
        assert_eq!(eq.to_array(Direction::Right), ["3", "+", "4", "=", "7"]);

        let eq = Equation::new(4, 3, Operation::Minus);
        assert_eq!(eq.to_array(Direction::Right), ["4", "-", "3", "=", "1"]);

        let eq = Equation::new(4, 3, Operation::Minus);
        assert_eq!(eq.to_array(Direction::Left), ["3", "-", "4", "=", "1"]);
    }

    #[test]
    fn test_get_start_position_for_addition() {
        let directions = [Direction::Left, Direction::Right, Direction::Down, Direction::Up];
        let width = 100;
        let param_position = 50;
        let eq = Equation::new(3, 4, Operation::Plus);

        let param = MatchParameter::X;
        for dir in directions {
            let pos = eq.get_start_position(width, dir, param, param_position);
            assert_eq!(param_position, pos);
        }

        let param = MatchParameter::Y;
        for dir in directions {
            let delta = get_direction_step(dir, width);
            let pos = eq.get_start_position(width, dir, param, param_position);
            assert_eq!(50 - 2*delta, pos as i32);
        }

        let param = MatchParameter::Result;
        for dir in directions {
            let delta = get_direction_step(dir, width);
            let pos = eq.get_start_position(width, dir, param, param_position);
            assert_eq!(50 - 4*delta, pos as i32);
        }
    }

    #[test]
    fn test_get_start_position_for_substraction() {
        let directions = [Direction::Left, Direction::Right, Direction::Down, Direction::Up];
        let width = 100;
        let param_position = 50;
        let eq = Equation::new(4, 3, Operation::Minus);

        let param = MatchParameter::X;
        for dir in [Direction::Right, Direction::Down] {
            let pos = eq.get_start_position(width, dir, param, param_position);
            assert_eq!(param_position, pos);
        }

        for dir in [Direction::Left, Direction::Up] {
            let delta = get_direction_step(dir, width);
            let pos = eq.get_start_position(width, dir, param, param_position);
            assert_eq!(50 - 2*delta, pos as i32);
        }

        let param = MatchParameter::Y;
        for dir in [Direction::Right, Direction::Down] {
            let delta = get_direction_step(dir, width);
            let pos = eq.get_start_position(width, dir, param, param_position);
            assert_eq!(50 - 2*delta, pos as i32);
        }

        for dir in [Direction::Left, Direction::Up] {
            let pos = eq.get_start_position(width, dir, param, param_position);
            assert_eq!(param_position, pos);
        }

        let param = MatchParameter::Result;
        for dir in directions {
            let delta = get_direction_step(dir, width);
            let pos = eq.get_start_position(width, dir, param, param_position);
            assert_eq!(50 - 4*delta, pos as i32);
        }
    }
}
