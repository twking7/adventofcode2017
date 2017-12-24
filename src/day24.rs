use std::collections::HashMap;

fn part1(s: &str) -> usize {
    let parts = str_to_tuples(s);

    add_part1(0, 0, &parts)
}

fn part2(s: &str) -> usize {
    let parts = str_to_tuples(s);
    let mut memo: HashMap<usize, usize> = HashMap::new();

    add_part2(0, 0, 0, &parts, &mut memo);

    let mut max_depth = 0;
    let mut max_score = 0;
    for (k, v) in memo {
        if k > max_depth {
            max_depth = k;
            max_score = v;
        }
        if k == max_depth && v > max_score {
            max_score = v;
        }
    }

    max_score
}

fn add_part1(score: usize, side: usize, parts: &[(usize, usize)]) -> usize {
    let usable_parts: Vec<&(usize, usize)> = parts
        .iter()
        .filter(|part| can_add(side, part))
        .collect();

    if usable_parts.is_empty() { return score }

    usable_parts.iter().map(|part| {
        let next_parts: Vec<(usize, usize)> = parts
            .iter()
            .filter(|p| p != part)
            .cloned()
            .collect();
        let next_side = if side == part.0 { part.1 } else { part.0 };
        add_part1(score + part.0 + part.1, next_side, &next_parts)
    }).max().unwrap()
}

fn add_part2(depth: usize, score: usize, side: usize, parts: &[(usize, usize)], mut memo: &mut HashMap<usize, usize>) -> usize {
    let usable_parts: Vec<&(usize, usize)> = parts
        .iter()
        .filter(|part| can_add(side, part))
        .collect();

    if usable_parts.is_empty() {
        let subscore = *memo.get(&depth).unwrap_or(&0);

        if subscore > score {
            memo.insert(depth, subscore);
        } else {
            memo.insert(depth, score);
        }
        return score
    }

    usable_parts.iter().map(|part| {
        let next_parts: Vec<(usize, usize)> = parts
            .iter()
            .filter(|p| p != part)
            .cloned()
            .collect();
        let next_side = if side == part.0 { part.1 } else { part.0 };
        add_part2(depth + 1, score + part.0 + part.1, next_side, &next_parts, &mut memo)
    }).max().unwrap()
}

fn can_add(side: usize, part: &(usize, usize)) -> bool {
    side == part.0 || side == part.1
}

fn str_to_tuples(s: &str) -> Vec<(usize, usize)> {
    s.lines().map(|line| {
        let v = line.split('/').collect::<Vec<&str>>();
        (v[0].parse().unwrap(), v[1].parse().unwrap())
    }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(part1(input), 31);
    }

    #[test]
    fn test_part2() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(part2(input), 19);
    }
}
