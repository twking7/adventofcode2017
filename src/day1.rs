fn summer(num: &str, rotate: usize) -> usize {
    let d1: Vec<usize> = num
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect();

    let mut d2 = d1.clone();
    d2.rotate(rotate);

    d1.iter()
        .zip(d2.iter())
        .filter(|&(x, y)| x == y)
        .map(|(x, _)| x)
        .sum()
}

fn part1(num: &str) -> usize {
    summer(num, 1)
}

fn part2(num: &str) -> usize {
    summer(num, num.len() / 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1122"), 3);
        assert_eq!(part1("1111"), 4);
        assert_eq!(part1("1234"), 0);
        assert_eq!(part1("91212129"), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1212"), 6);
        assert_eq!(part2("1221"), 0);
        assert_eq!(part2("123425"), 4);
        assert_eq!(part2("123123"), 12);
        assert_eq!(part2("12131415"), 4);
    }
}
