use crate::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

pub fn get_random_match_parameter() -> MatchParameter {
    let mut rng = rand::thread_rng();
    let proba: f64 = rng.gen();
    if proba < 0.33 {
        return MatchParameter::Result;
    } else if proba < 0.66 {
        return MatchParameter::Y;
    }
    MatchParameter::X
}

// On va faire simple dans un premier temps :
// - On ne regarde pas s'il est possible de faire une liaison ici avec le résultat
pub fn is_insertion_possible(
    width: i32,
    grid: &Vec<String>,
    position: usize,
    former_dir: Direction,
    match_param: MatchParameter,
    eq: &Equation
) -> Option<Direction> {

    let directions = match former_dir {
        Direction::Down | Direction::Up => [Direction::Left, Direction::Right],
        Direction::Right | Direction::Left => [Direction::Down, Direction::Up],
    };

    let match_position = position;

    // log!("Trying : {}", eq.to_string());

    for dir in directions {
        let delta = get_direction_step(dir, width);
        let origin = eq.get_start_position(width, dir, match_param, match_position);
        for i in 0..=4 {
            let position = (origin as i32) + i * delta;
            // log!("Cell {position} ({i})");

            if position < 0 {
                break;
            }

            let position = position as usize;
            if position > grid.len() {
                break;
            }

            // log!("{}", grid[position]);

            if grid[position] != " " && position != match_position {
                return None;
            }
            
            if i == 4 {
                return Some(dir);
            }
        }
    }
    None
}

pub fn insert_equation(
    width: i32,
    grid: &mut Vec<String>,
    start_position: usize,
    dir: Direction,
    eq: Equation,
    marked: &mut Vec<(usize, Direction)>,
) {
    let delta = get_direction_step(dir, width);

    // log!("{}", eq.to_string());

    let representation = eq.to_array(dir);
    for (idx, term) in representation.iter().enumerate() {
        let position = (start_position as i32 + (idx as i32 * delta)) as usize;
        if position > grid.len() {
            let direction = match dir {
                Direction::Down => "Down",
                Direction::Up => "Up",
                Direction::Left => "Left",
                Direction::Right => "Right",
            };
            panic!(
                "Index out of bounds : dir={} delta={} start={} size={}",
                direction, delta, start_position, width
            );
        }
        grid[position] = term.to_string();
        match term.trim().parse::<u32>() {
            Ok(_) => marked.push((position, dir)),
            Err(_) => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_insertion_possible() {
        let eq = Equation::new(3, 4, Operation::Plus);
        let grid: Vec<String> = (0..100).map(|_| " ".to_string()).collect();
        match is_insertion_possible(100, &grid, 50, Direction::Up, MatchParameter::X, &eq) {
            Some(_) => {}
            None => assert!(false, "The insertion is not working"),
        }
    }
}
