use std::fs;

#[derive(Debug)]
struct XY {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    points: Vec<Vec<bool>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut points = Vec::new();
        for line in value.lines() {
            let mut temp = Vec::new();
            for char in line.chars() {
                temp.push(char == '@');
            }
            points.push(temp)
        }
        Grid { points }
    }
}

impl Grid {
    fn height(&self) -> usize {
        self.points.len()
    }

    fn width(&self) -> usize {
        self.points[0].len()
    }

    fn neighbours(&self, xy: &XY) -> Vec<XY> {
        [-1, 0, 1]
            .into_iter()
            .flat_map(move |dx| {
                [-1, 0, 1].into_iter().flat_map(move |dy| {
                    (dx != 0 || dy != 0).then_some(XY {
                        x: xy.x.checked_add_signed(dx)?,
                        y: xy.y.checked_add_signed(dy)?,
                    })
                })
            })
            .filter(|xy| xy.x < self.width() && xy.y < self.height())
            .collect()
    }

    fn accessible(&self, xy: &XY) -> bool {
        if !self.points[xy.y][xy.x] {
            return false;
        }
        let mut count = 0;
        for neighbour in self.neighbours(xy) {
            if self.points[neighbour.y][neighbour.x] {
                count += 1;
            }
        }
        count < 4
    }

    fn remove(&mut self, xys: Vec<XY>) {
        for xy in xys {
            self.points[xy.y][xy.x] = false;
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from(input);
    let mut total = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.accessible(&XY { x, y }) {
                total += 1;
            }
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut total = 0;
    let mut removed = true;
    while removed {
        removed = false;
        let mut removals = Vec::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let xy = XY { x, y };
                if grid.accessible(&xy) {
                    removals.push(xy)
                }
            }
        }
        if removals.len() > 0 {
            removed = true;
            total += removals.len();
            grid.remove(removals);
        }
    }
    total
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
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(43, part2(&input));
    }
}
