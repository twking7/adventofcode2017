fn part1(mut a: usize, mut b: usize, cycles: usize) -> usize {
    let a_factor = 16807;
    let b_factor = 48271;
    let divisor = 2147483647;
    let mut matches = 0;

    for _ in 0..cycles {
        if low16(a) == low16(b) {
            matches += 1;
        }

        a = a * a_factor % divisor;
        b = b * b_factor % divisor;
    }

    matches
}

fn part2(mut a: usize, mut b: usize, cycles: usize) -> usize {
    let a_factor = 16807;
    let b_factor = 48271;
    let divisor = 2147483647;
    let mut valid_a = Vec::new();
    let mut valid_b = Vec::new();
    let mut matches = 0;

    while valid_a.len() < cycles {
        if a % 4 == 0 {
            valid_a.push(a);
        }

        a = a * a_factor % divisor;
    }

    while valid_b.len() < cycles {
        if b % 8 == 0 {
            valid_b.push(b);
        }

        b = b * b_factor % divisor;
    }

    for i in 0..cycles {
        if low16(valid_a[i]) == low16(valid_b[i]) {
            matches += 1;
        }
    }

    matches
}

fn low16(n: usize) -> usize {
    n & 0b1111111111111111
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(65, 8921, 40_000_000), 588);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(65, 8921, 1056), 1);
        assert_eq!(part2(65, 8921, 5_000_000), 309);
    }
}
