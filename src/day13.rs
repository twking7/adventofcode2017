fn part1(s: &str) -> usize {
    let areas = areas_from_str(s);
    let mut pos = 0;
    let mut score = 0;
    let max_range: usize = areas.iter().map(|area| area.0).max().unwrap();

    while pos <= max_range {
        if let Some(area) = areas.iter().find(|area| area.0 == pos) {
            if is_caught(pos, area.1) {
                score += area.1 * pos;
            }
        }

        pos += 1;
    }

    score
}

fn part2(s: &str) -> usize {
    let areas = areas_from_str(s);
    let max_range: usize = areas.iter().map(|area| area.0).max().unwrap();
    let mut delay = 0;
    let mut caught = true;

    while caught {
        let mut pos = 0;
        caught = false;

        while !caught && pos <= max_range {
            if let Some(area) = areas.iter().find(|area| area.0 == pos) {
                if is_caught(delay + pos, area.1) {
                    caught = true;
                    delay += 1;
                }
            }

            pos += 1;
        }
    }

    delay
}

fn is_caught(picosecond: usize, depth: usize) -> bool {
    picosecond % ((depth - 1) * 2) == 0
}

fn areas_from_str(s: &str) -> Vec<(usize, usize)> {
    s.split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.split(": ").map(|n| n.parse().unwrap()).collect::<Vec<usize>>())
        .map(|v| (v[0], v[1]))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_areas_from_str() {
        let input = "0: 3
10: 2
4: 40
60: 40";
        let expected = vec![(0, 3), (10, 2), (4, 40), (60, 40)];
        assert_eq!(areas_from_str(input), expected);
    }

    #[test]
    fn test_part1() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(part1(input), 24);
    }

    #[test]
    fn test_part2() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(part2(input), 10);
    }
}
