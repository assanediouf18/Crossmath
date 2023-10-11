use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// On va faire simple dans un premier temps :
// - On ne regarde pas s'il est possible de faire une liaison ici avec le résultat
pub fn is_insertion_possible(
    width: i32,
    grid: &Vec<String>,
    position: usize,
    former_dir: Direction
) -> Option<(Direction, MatchParameter)> {
    let directions = match former_dir {
        Direction::Down | Direction::Up => [Direction::Left, Direction::Right],
        Direction::Right | Direction::Left => [Direction::Down, Direction::Up],
    };
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

pub fn insert_equation(width: i32, 
    grid: &mut Vec<String>, 
    start_position: usize, 
    dir: Direction, 
    eq: Equation, 
    marked: &mut Vec<(usize, Direction)>) {
    let delta = get_direction_step(dir, width);

    // log!("{}", eq.to_string());
    
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
            Ok(_) => marked.push((position, dir)),
            Err(_) => {}
        };
    }
}

pub fn get_direction_step(dir: Direction, width: i32) -> i32 {
    match dir {
        Direction::Down => width,
        Direction::Up => -width,
        Direction::Left => -1,
        Direction::Right => 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_insertion_possible() {
        let grid: Vec<String> = (0..100).map(|_| " ".to_string()).collect();
        match is_insertion_possible(100, &grid, 50, Direction::Up) {
            Some(_) => {},
            None => assert!(false, "The insertion is not working")
        }
    }

    #[test]
    fn test_insertion() {
        let eq = Equation::new(3, 4, Operation::Plus);
        assert_eq!(eq.to_array(), ["3", "+", "4", "=", "7"]);
    }
}