fn part1(s: &str) -> usize {
    let mut score = 0;
    let mut depth = 0;
    let mut ignore = false;
    let mut garbage = false;

    for c in s.chars() {
        if ignore {
            ignore = false;
            continue;
        }
        match c {
            '{' => {
                if !garbage {
                    depth += 1;
                    score += depth;
                }
            },
            '}' => {
                if !garbage {
                    depth -= 1;
                }
            },
            '!' => { ignore = true; },
            '<' => { garbage = true; },
            '>' => { garbage = false; },
            _ => {},
        }
    }

    score
}

fn part2(s: &str) -> usize {
    let mut chars = 0;
    let mut garbage = false;
    let mut ignore = false;

    for c in s.chars() {
        if ignore {
            ignore = false;
            continue;
        }

        match c {
            '<' => {
                if !garbage {
                    garbage = true;
                } else {
                    chars += 1;
                }
            },
            '!' => { ignore = true; }
            '>' => { garbage = false; },
            _ => { if garbage { chars += 1; }},
        }
    }

    chars
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("{}"), 1);
        assert_eq!(part1("{{{}}}"), 6);
        assert_eq!(part1("{{},{}}"), 5);
        assert_eq!(part1("{{{},{},{{}}}}"), 16);
        assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(part1("{{<!>},{<!>},{<!>},{<a>}}"), 3);
        assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<a!>}}"), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("<>"), 0);
        assert_eq!(part2("<random characters>"), 17);
        assert_eq!(part2("<<<<>"), 3);
        assert_eq!(part2("<{!>}>"), 2);
        assert_eq!(part2("<!!>"), 0);
        assert_eq!(part2("<!!!>>"), 0);
        assert_eq!(part2("<{o\"i!a,<{i<a>"), 10);
    }
}
