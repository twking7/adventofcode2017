use std::collections::HashMap;

#[derive(Debug)]
enum Health {
    Clean,
    Weak,
    Infected,
    Flagged,
}

#[derive(Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn vector(&self) -> (isize, isize) {
        match *self {
            Dir::N => (0, 1),
            Dir::S => (0, -1),
            Dir::E => (1, 0),
            Dir::W => (-1, 0),
        }
    }

    fn turn_left(self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
        }
    }

    fn turn_right(self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    fn reverse(self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

fn part1(s: &str, cycles: usize) -> usize {
    let mut infects = 0;
    let rows = s.split('\n').count() as isize;
    let cols = s.split('\n').nth(0).unwrap().len() as isize;
    let mut p = (cols / 2, -rows / 2);
    let mut d = Dir::N;
    let mut grid = str_to_hash(s);

    for _ in 0..cycles {
        match *grid.get(&p).unwrap_or(&Health::Clean) {
            Health::Infected => {
                d = d.turn_right();
                grid.insert(p, Health::Clean);
            },
            _ => {
                d = d.turn_left();
                grid.insert(p, Health::Infected);
                infects += 1;
            }
        }

        let v = d.vector();
        p = (p.0 + v.0, p.1 + v.1);
    }

    infects
}

fn part2(s: &str, cycles: usize) -> usize {
    let mut infects = 0;
    let rows = s.split('\n').count() as isize;
    let cols = s.split('\n').nth(0).unwrap().len() as isize;
    let mut p = (cols / 2, -rows / 2);
    let mut d = Dir::N;
    let mut grid = str_to_hash(s);

    for _ in 0..cycles {
        match *grid.get(&p).unwrap_or(&Health::Clean) {
            Health::Infected => {
                d = d.turn_right();
                grid.insert(p, Health::Flagged);
            },
            Health::Weak => {
                grid.insert(p, Health::Infected);
                infects += 1;
            },
            Health::Flagged => {
                d = d.reverse();
                grid.insert(p, Health::Clean);
            },
            Health::Clean => {
                d = d.turn_left();
                grid.insert(p, Health::Weak);
            }
        }

        let v = d.vector();
        p = (p.0 + v.0, p.1 + v.1);
    }

    infects
}

fn str_to_hash(s: &str) -> HashMap<(isize, isize), Health> {
    let matrix = s.split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut h = HashMap::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == '#' {
                let x = j as isize;
                let y = i as isize;
                h.insert((x, -y), Health::Infected);
            }
        }
    }

    h
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..#\n#..\n...";
        assert_eq!(part1(input, 7), 5);
        assert_eq!(part1(input, 70), 41);
    }

    #[test]
    fn test_part2() {
        let input = "..#\n#..\n...";
        assert_eq!(part2(input, 100), 26);
    }
}
