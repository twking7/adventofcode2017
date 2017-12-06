use std::collections::HashSet;
use std::iter::FromIterator;

fn part1(s: &str) -> usize {
    s.split('\n')
        .filter(|phrase| {
            let words = phrase.split_whitespace().collect::<Vec<&str>>();
            let set: HashSet<&&str> = HashSet::from_iter(words.iter());
            words.len() == set.len()
        })
        .collect::<Vec<&str>>()
        .len()
}

fn part2(s: &str) -> usize {
    s.split('\n')
        .filter(|phrase| {
            let words = phrase
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars().collect::<Vec<char>>();
                    chars.sort_by(|a, b| b.cmp(a));
                    String::from_iter(chars)
                })
                .collect::<Vec<String>>();

            let set: HashSet<&String> = HashSet::from_iter(words.iter());
            words.len() == set.len()
        })
        .collect::<Vec<&str>>()
        .len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("aa bb cc dd"), 1);
        assert_eq!(part1("aa bb aa dd"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("abcde fghij"), 1);
        assert_eq!(part2("abcde xyz ecdab"), 0);
    }
}
