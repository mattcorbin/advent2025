use std::fs;

enum Direction {
    L,
    R
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Unknown direction")
        }
    }
}

fn parse_input(input: &str) -> Vec<(Direction, isize)> {
    let mut retval = Vec::new();
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        retval.push(
            (
                Direction::from(chars[0]),
                String::from_iter(&chars[1..]).parse().unwrap()
            )
        );
    }
    retval
}


fn part1(input: &str) -> usize {
    let mut val = 50;
    let mut count = 0;
    let actions = parse_input(input);
    for action in actions {
        match action.0 {
            Direction::L => {
                val =  (val - action.1) % 100;
                if val == 0 {
                    count += 1;
                }
            },
            Direction::R => {
                val =  (val + action.1) % 100;
                if val == 0 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let mut val = 50;
    let mut count = 0;
    let actions = parse_input(input);
    for action in actions {
        match action.0 {
            Direction::L => {
                let new_val = val - action.1;
                if new_val < 0 && (new_val / 100).abs() < 100  {
                    count += 1;
                } else {
                    count += (new_val / 100).abs() as usize;
                }
                val = new_val % 100;
            },
            Direction::R => {
                let new_val = val + action.1;
                count += (new_val / 100).abs() as usize;
                val = new_val % 100;
            }
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(6, part2(&input));
    }

    #[test]
    fn test_p2_silly() {
        let input = "R1000";
        assert_eq!(10, part2(&input));
    }
}
