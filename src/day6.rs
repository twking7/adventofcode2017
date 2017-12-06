use std::collections::HashSet;

fn part1(blocks: Vec<usize>) -> usize {
    let mut v = blocks.clone();
    compute_cycles(&mut v)
}

fn part2(blocks: Vec<usize>) -> usize {
    let mut v = blocks.clone();
    compute_cycles(&mut v);
    compute_cycles(&mut v)
}

fn compute_cycles(v: &mut Vec<usize>) -> usize {
    let mut cycles = 0;
    let mut seen = HashSet::new();
    let mut do_cycle = true;
    let l = v.len();
    seen.insert(v.clone());

    while do_cycle {
        cycles += 1;
        let max_val = v.iter().max().unwrap().clone();
        let max_idx = v.iter().position(|elem| *elem == max_val).unwrap();

        let mut rem = max_val;
        let mut i = max_idx + 1;
        v[max_idx] = 0;
        while rem > 0 {
            v[i % l] += 1;
            rem -= 1;
            i += 1;
        }
        do_cycle = seen.get(v).is_none();
        seen.insert(v.clone());
    }

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![0, 2, 7, 0];
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part2() {
        let input = vec![2, 4, 1, 2];
        assert_eq!(part2(input), 4);
    }
}
