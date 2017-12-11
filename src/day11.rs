use std::ops::AddAssign;

#[derive(Debug)]
struct Point(isize, isize);

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point(self.0 + other.0, self.1 + other.1);
    }
}

fn part1(input: &str) -> usize {
    let dirs = split_str(input);
    let mut p = Point(0, 0);

    for dir in dirs {
        match dir {
            "n"  => { p += Point(0, 2); },
            "s"  => { p += Point(0, -2); },
            "ne" => { p += Point(1, 1); },
            "sw" => { p += Point(-1, -1); },
            "se" => { p += Point(1, -1); },
            "nw" => { p += Point(-1, 1); }
            _ => panic!("unexpected direction {}", dir),
        };
    }

    let mut steps = 0;
    p.0 = p.0.abs();
    p.1 = p.1.abs();
    while p.0 != 0 && p.1 != 0 {
        steps += 1;
        p += Point(-1, -1);
    }
    steps += (p.0.abs() + p.1.abs()) / 2;
    steps as usize
}

fn part2(input: &str) -> usize {
    let dirs = split_str(input);
    let mut p = Point(0, 0);
    let mut max_dist = 0;

    for dir in dirs {
        match dir {
            "n"  => { p += Point(0, 2); },
            "s"  => { p += Point(0, -2); },
            "ne" => { p += Point(1, 1); },
            "sw" => { p += Point(-1, -1); },
            "se" => { p += Point(1, -1); },
            "nw" => { p += Point(-1, 1); }
            _ => panic!("unexpected direction {}", dir),
        };

        let mut steps = 0;
        let mut temp_p = Point(p.0.abs(), p.1.abs());
        while temp_p.0 != 0 && temp_p.1 != 0 {
            steps += 1;
            temp_p += Point(-1, -1);
        }
        steps += (temp_p.0.abs() + temp_p.1.abs()) / 2;

        if steps as usize > max_dist {
            max_dist = steps as usize;
        }
    }

    max_dist
}

fn split_str(s: &str) -> Vec<&str> {
    s.split(',').collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ne,ne,ne"), 3);
        assert_eq!(part1("ne,ne,sw,sw"), 0);
        assert_eq!(part1("ne,ne,s,s"), 2);
        assert_eq!(part1("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("ne,ne,ne"), 3);
        assert_eq!(part2("ne,ne,sw,sw"), 2);
    }
}
