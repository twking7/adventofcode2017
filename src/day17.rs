fn part1(steps: usize) -> usize {
    let mut v = vec![0];
    let mut curr = 0;

    for i in 1..2018 {
        curr = (curr + steps) % v.len();

        if curr == v.len() - 1 {
            v.push(i);
        } else {
            v.insert(curr + 1, i);
        }

        curr += 1;
    }

    let idx = v.iter().position(|x| *x == 2017).unwrap();
    v[idx + 1]
}

fn part2(steps: usize) -> usize {
    let mut curr = 0;
    let mut zero_idx = 0;
    let mut next_num = 0;

    for i in 1..50_000_001 {
        curr = (curr + steps) % i;

        if curr + 1 <= zero_idx {
            zero_idx += 1;
        }

        if curr + 1 == zero_idx + 1 {
            next_num = i;
        }

        curr += 1;
    }

    next_num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(3), 638);
    }
}
