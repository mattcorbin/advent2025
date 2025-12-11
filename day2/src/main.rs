use std::fs;

struct IDRange {
    min: usize,
    max: usize,
}

struct IDRanges(Vec<IDRange>);

impl From<&str> for IDRange {
    fn from(value: &str) -> Self {
        let values = value
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        IDRange {
            min: values[0],
            max: values[1],
        }
    }
}

impl From<&str> for IDRanges {
    fn from(value: &str) -> Self {
        let mut retval = Vec::new();
        for item in value.split(",") {
            retval.push(IDRange::from(item));
        }
        IDRanges(retval)
    }
}

fn to_digits(mut v: usize) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::with_capacity(20);

    while v > 0 {
        let n = (v % 10) as u8;
        v /= 10;
        digits.push(n);
    }
    digits.reverse();
    digits
}

fn is_all_same<T: Eq>(slice: &[T]) -> bool {
    slice
        .get(0)
        .map(|first| slice.iter().all(|x| x == first))
        .unwrap_or(true)
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    let ranges = IDRanges::from(input);
    for range in ranges.0 {
        for i in range.min..=range.max {
            let digits = to_digits(i);
            let len = digits.len();
            if len % 2 != 0 {
                continue;
            }
            if digits[..len/2] == digits[len/2..] {
                sum += i;
            }
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    let ranges = IDRanges::from(input);
    for range in ranges.0 {
        for i in range.min..=range.max {
            let digits = to_digits(i);
            for chunk_size in 1..digits.len() {
                let windows = digits.chunks(chunk_size).collect::<Vec<&[u8]>>();
                if is_all_same(windows.as_slice()) {
                    sum += i;
                    break;
                }
            }
        }
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
        assert_eq!(1227775554, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(4174379265, part2(&input));
    }
}
