fn part1(s: &str) -> usize {
    let mut v: Vec<isize> = s.split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let mut curr_idx = 0;
    let mut steps = 0;
    while curr_idx >= 0 && curr_idx < v.len() as isize {
        let jmp = v[curr_idx as usize];
        v[curr_idx as usize] += 1;
        curr_idx += jmp;
        steps += 1;
    }

    steps
}

fn part2(s: &str) -> usize {
    let mut v: Vec<isize> = s.split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let mut curr_idx = 0;
    let mut steps = 0;
    while curr_idx >= 0 && curr_idx < v.len() as isize {
        let jmp = v[curr_idx as usize];
        if jmp >= 3 {
            v[curr_idx as usize] -= 1;
        } else {
            v[curr_idx as usize] += 1;
        }
        curr_idx += jmp;
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("0 3 0 1 -3"), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("0 3 0 1 -3"), 10);
    }
}
