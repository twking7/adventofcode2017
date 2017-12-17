use std::collections::HashMap;

enum Step<'a> {
    Shift(usize),
    Swap(usize, usize),
    Partner(&'a str, &'a str),
}

impl<'a> From<&'a str> for Step<'a> {
    fn from(s: &'a str) -> Self {
        match s.chars().nth(0).unwrap() {
            's' => Step::Shift(s[1..].parse().unwrap()),
            'x' => {
                let nums = s[1..].split('/').map(|n| n.parse().unwrap()).collect::<Vec<usize>>();
                Step::Swap(nums[0], nums[1])
            },
            _ => {
                let nums = s[1..].split('/').collect::<Vec<&str>>();
                Step::Partner(nums[0], nums[1])
            }
        }
    }
}

fn part1(s: &str, moves: &str) -> String {
    let mut v: Vec<char> = s.chars().collect();
    let len = v.len();
    let steps = moves.split(',').map(|m| Step::from(m)).collect::<Vec<Step>>();

    for step in steps {
        match step {
            Step::Shift(n) => v.rotate(len - n),
            Step::Swap(a, b) => v.swap(a, b),
            Step::Partner(a, b) => {
                let idx_a = v.iter().position(|c| c == &a.chars().nth(0).unwrap()).unwrap();
                let idx_b = v.iter().position(|c| c == &b.chars().nth(0).unwrap()).unwrap();
                v.swap(idx_a, idx_b)
            },
        }
    }

    v.into_iter().collect()
}

fn part2(s: &str, moves: &str, cycles: usize) -> String {
    let mut v: Vec<String> = s.chars().map(|c| c.to_string()).collect();
    let len = v.len();
    let steps = moves.split(',').map(|m| Step::from(m)).collect::<Vec<Step>>();
    let mut memo: HashMap<Vec<String>, usize> = HashMap::new();

    for i in 0..cycles {
        if memo.get(&v).is_some() { break }

        memo.insert(v.to_vec(), i);

        for step in &steps {
            match step {
                &Step::Shift(n) => v.rotate(len - n),
                &Step::Swap(a, b) => v.swap(a, b),
                &Step::Partner(a, b) => {
                    let idx_a = v.iter().position(|c| c == a).unwrap();
                    let idx_b = v.iter().position(|c| c == b).unwrap();
                    v.swap(idx_a, idx_b)
                },
            }
        }
    }

    if memo.len() < cycles {
        let final_idx = cycles % memo.len();
        let (result, _) = memo.iter().find(|&(_, i)| *i == final_idx).unwrap();
        result.join("")
    } else {
        v.join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcde", "s1,x3/4,pe/b"), "baedc");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("abcde", "s1,x3/4,pe/b", 2), "ceadb");
    }
}
