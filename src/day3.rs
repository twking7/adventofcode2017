use std::collections::HashMap;

type Point = (isize, isize);

fn part1(n: usize) -> usize {
    let mut p: Point = (0, 0);
    let mut d: Point = (1, 0);
    let mut go_up_coord: Point = d;
    let mut m = 1;

    while m < n {
        p.0 += d.0;
        p.1 += d.1;

        if p == go_up_coord {
            d = (0, 1);
        } else if p.0.abs() == p.1.abs() {
            if p.0 > 0 && p.1 > 0 {
                d = (-1, 0);
            } else if p.0 < 0 && p.1 > 0 {
                d = (0, -1);
            } else if p.0 < 0 && p.1 < 0 {
                d = (1, 0);
            } else {
                d = (1, 0);
                go_up_coord = (p.0 + 1, p.1);
            }
        }

        m += 1;
    }

    p.0.abs() as usize + p.1.abs() as usize
}

fn part2(n: usize) -> usize {
    let mut p: Point = (0, 0);
    let mut d: Point = (1, 0);
    let mut go_up_coord: Point = d;
    let mut m = 1;
    let mut h: HashMap<Point, usize> = HashMap::new();
    h.insert(p, 1);

    while m <= n {
        p.0 += d.0;
        p.1 += d.1;
        m = sum_neighbors(&h, &p);
        h.insert(p, m);

        if p == go_up_coord {
            d = (0, 1);
        } else if p.0.abs() == p.1.abs() {
            if p.0 > 0 && p.1 > 0 {
                d = (-1, 0);
            } else if p.0 < 0 && p.1 > 0 {
                d = (0, -1);
            } else if p.0 < 0 && p.1 < 0 {
                d = (1, 0);
            } else {
                d = (1, 0);
                go_up_coord = (p.0 + 1, p.1);
            }
        }
    }

    m
}

fn sum_neighbors(h: &HashMap<Point, usize>, p: &Point) -> usize {
    h.get(&(p.0 + 1, p.1)).unwrap_or(&0) +
        h.get(&(p.0 + 1, p.1 + 1)).unwrap_or(&0) +
        h.get(&(p.0, p.1 + 1)).unwrap_or(&0) +
        h.get(&(p.0 - 1, p.1 + 1)).unwrap_or(&0) +
        h.get(&(p.0 - 1, p.1)).unwrap_or(&0) +
        h.get(&(p.0 - 1, p.1 - 1)).unwrap_or(&0) +
        h.get(&(p.0, p.1 - 1)).unwrap_or(&0) +
        h.get(&(p.0 + 1, p.1 - 1)).unwrap_or(&0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(1), 0);
        assert_eq!(part1(12), 3);
        assert_eq!(part1(23), 2);
        assert_eq!(part1(1024), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(1), 2);
        assert_eq!(part2(2), 4);
        assert_eq!(part2(10), 11);
    }
}
