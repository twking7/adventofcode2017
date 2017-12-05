fn part1(s: &str) -> usize {
    s.split("\n").map(|line| {
        let mut nums = line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        nums.sort();
        nums.last().unwrap() - nums.first().unwrap()
    }).sum()
}

fn part2(s: &str) -> usize {
    s.split("\n").fold(Vec::new(), |mut v, line| {
        let mut nums = line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();

        nums.sort();
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[j] % nums[i] == 0 {
                    v.push(nums[j] / nums[i]);
                }
            }
        }
        v
    }).iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let sheet = "5 1 9 5\n7 5 3\n2 4 6 8";
        assert_eq!(part1(sheet), 18);
    }

    #[test]
    fn test_part2() {
        let sheet = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        assert_eq!(part2(sheet), 9);
    }
}
