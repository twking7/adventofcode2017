#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Point(usize, usize);

fn part1(maze: &str) -> String {
    let matrix = maze_to_matrix(maze);
    let mut chars = Vec::new();

    let mut d = Direction::Down;
    let mut pos = Point(0, matrix[0].iter().position(|c| *c == '|').unwrap());

    loop {
        match matrix[pos.0][pos.1] {
            '|' | '-' => pos = step(&pos, &d),
            '+' => {
                d = turn(&pos, &d, &matrix);
                pos = step(&pos, &d);
            },
            ' ' => break,
            c => {
                chars.push(c);
                pos = step(&pos, &d)
            },
        };
    }

    chars.into_iter().collect::<String>()
}

fn part2(maze: &str) -> usize {
    let matrix = maze_to_matrix(maze);
    let mut steps = 0;

    let mut d = Direction::Down;
    let mut pos = Point(0, matrix[0].iter().position(|c| *c == '|').unwrap());

    loop {
        match matrix[pos.0][pos.1] {
            '+' => {
                d = turn(&pos, &d, &matrix);
                pos = step(&pos, &d);
            },
            ' ' => break,
            _ => pos = step(&pos, &d),
        };
        steps += 1;
    }

    steps
}

fn turn(pos: &Point, d: &Direction, matrix: &[Vec<char>]) -> Direction {
    let w = matrix[0].len();

    if *d != Direction::Right && pos.1 > 0 && matrix[pos.0][pos.1 - 1] != ' ' {
        Direction::Left
    } else if *d != Direction::Left && pos.1 < w - 1 && matrix[pos.0][pos.1 + 1] != ' ' {
        Direction::Right
    } else if *d != Direction::Down && pos.0 > 0 && matrix[pos.0 - 1][pos.1] != ' ' {
        Direction::Up
    } else {
        Direction::Down
    }
}

fn step(pos: &Point, d: &Direction) -> Point {
    match *d {
        Direction::Down => Point(pos.0 + 1, pos.1),
        Direction::Up => Point(pos.0 - 1, pos.1),
        Direction::Left => Point(pos.0, pos.1 - 1),
        Direction::Right => Point(pos.0, pos.1 + 1),
    }
}

fn maze_to_matrix(maze: &str) -> Vec<Vec<char>> {
    maze.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

        assert_eq!(part1(input), "ABCDEF");
    }

    #[test]
    fn test_part2() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

        assert_eq!(part2(input), 38);
    }
}
