use std::collections::HashMap;

fn part1(input: &str) -> isize {
    let expressions = parse_lines(input);
    let mut registers: HashMap<&str, isize> = HashMap::new();

    for expression in expressions {
        let if_result = {
            let if_var = registers.entry(expression[4]).or_insert(0);
            let if_op = expression[5];
            let if_val: isize = expression[6].parse().unwrap();
            do_if(*if_var, if_op, if_val)
        };

        if if_result {
            let var = *registers.entry(expression[0]).or_insert(0);
            let val: isize = expression[2].parse().unwrap();

            match expression[1] {
                "inc" => registers.insert(expression[0], var + val),
                "dec" => registers.insert(expression[0], var - val),
                _ => panic!("unimplemented operator: {}", expression[1]),
            };
        }
    }

    *registers.values().max().unwrap_or(&0)
}

fn part2(input: &str) -> isize {
    let expressions = parse_lines(input);
    let mut registers: HashMap<&str, isize> = HashMap::new();
    let mut max = 0;

    for expression in expressions {
        let if_result = {
            let if_var = registers.entry(expression[4]).or_insert(0);
            let if_op = expression[5];
            let if_val: isize = expression[6].parse().unwrap();
            do_if(*if_var, if_op, if_val)
        };

        if if_result {
            let var = *registers.entry(expression[0]).or_insert(0);
            let val: isize = expression[2].parse().unwrap();

            match expression[1] {
                "inc" => {
                    let new_val = var + val;
                    if new_val > max {
                        max = new_val;
                    }
                    registers.insert(expression[0], new_val)
                },
                "dec" => {
                    let new_val = var - val;
                    if new_val > max {
                        max = new_val;
                    }
                    registers.insert(expression[0], new_val)
                }
                _ => panic!("unimplemented operator: {}", expression[1]),
            };
        }
    }

    max
}

fn do_if(var: isize, op: &str, val: isize) -> bool {
    match op {
        ">" => var > val,
        "<" => var < val,
        ">=" => var >= val,
        "<=" => var <= val,
        "==" => var == val,
        "!=" => var != val,
        _ => panic!("unimplemented comparison operator: {}", op),
    }
}

fn parse_lines(input: &str) -> Vec<Vec<&str>> {
    input.split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_part2() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(part2(input), 10);
    }
}
