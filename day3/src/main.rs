use std::fs;

struct BatteryBank {
    batteries: Vec<usize>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        BatteryBank {
            batteries: value
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
        }
    }
}

struct BatteryBanks {
    banks: Vec<BatteryBank>,
}

impl From<&str> for BatteryBanks {
    fn from(value: &str) -> Self {
        BatteryBanks {
            banks: value.lines().map(|x| BatteryBank::from(x)).collect(),
        }
    }
}

fn part1(input: &str) -> usize {
    let banks = BatteryBanks::from(input);
    let mut sum = 0;
    for bank in &banks.banks {
        let mut first = 0;
        let mut max_idx = 0;
        for i in 0..(bank.batteries.len() - 1) {
            if bank.batteries[i] > first {
                first = bank.batteries[i];
                max_idx = i;
            }
        }
        let mut second = 0;
        for i in max_idx + 1..bank.batteries.len() {
            if bank.batteries[i] > second {
                second = bank.batteries[i];
            }
        }
        sum += format!("{first}{second}").parse::<usize>().unwrap();
    }
    sum
}

fn part2(input: &str) -> usize {
    let banks = BatteryBanks::from(input);
    let mut sum = 0;
    for bank in &banks.banks {
        let mut digits = vec![0; 12];
        let mut indices = vec![0; 12];
        for i in (0..=11).rev() {
            let idx = 11 - i;
            let start;
            if idx == 0 {
                start = 0;
            } else {
                start = indices[idx - 1] + 1;
            }
            for j in start..(bank.batteries.len() - i) {
                if bank.batteries[j] > digits[idx] {
                    digits[idx] = bank.batteries[j];
                    indices[idx] = j;
                }
            }
        }
        sum += digits
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<usize>()
            .unwrap();
    }
    sum
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
        assert_eq!(357, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(3121910778619, part2(&input));
    }
}
