use std::collections::HashMap;

fn part1(s: &str) -> isize {
    let mut reg: HashMap<&str, isize> = HashMap::new();
    let lines: Vec<&str> = s.lines().collect();
    let mut ptr = 0isize;
    let mut muls = 0;

    loop {
        let asm: Vec<&str> = lines[ptr as usize].split_whitespace().collect();
        ptr += 1;

        let lop = match asm[1].parse::<isize>() {
            Ok(i) => i,
            Err(_) => *reg.get(asm[1]).unwrap_or(&0),
        };

        let rop = match asm[2].parse::<isize>() {
            Ok(i) => i,
            Err(_) => *reg.get(asm[2]).unwrap_or(&0),
        };

        match asm[0] {
            "set" => { reg.insert(asm[1], rop); },
            "sub" => { reg.insert(asm[1], lop - rop); },
            "mul" => { 
                muls += 1;
                reg.insert(asm[1], lop * rop);
            },
            "jnz" => { if lop != 0 { ptr += rop - 1 } },
            _ => panic!("unexpected op: {}", asm[0]),
        }

        if ptr < 0 || (ptr as usize) >= lines.len() { return muls }
    }
}

fn part2(s: &str) -> isize {
    let mut reg: HashMap<&str, isize> = HashMap::new();
    let lines: Vec<&str> = s.lines().collect();
    let mut ptr = 0isize;
    let mut h = 0;

    loop {
        let line = lines[ptr as usize];
        let asm: Vec<&str> = line.split_whitespace().collect();
        ptr += 1;

        let lop = match asm[1].parse::<isize>() {
            Ok(i) => i,
            Err(_) => *reg.get(asm[1]).unwrap_or(&0),
        };

        let rop = match asm[2].parse::<isize>() {
            Ok(i) => i,
            Err(_) => *reg.get(asm[2]).unwrap_or(&0),
        };

        match asm[0] {
            "set" => { reg.insert(asm[1], rop); },
            "sub" => { reg.insert(asm[1], lop - rop); },
            "pri" => { if !is_prime(rop) { h += 1 }},
            "mul" => { reg.insert(asm[1], lop * rop); },
            "jnz" => { if lop != 0 { ptr += rop - 1 } },
            _ => panic!("unexpected op: {}", asm[0]),
        }

        if ptr < 0 || (ptr as usize) >= lines.len() { return h }
    }
}

fn is_prime(x: isize) -> bool {
    for i in 2..(x/2) {
        if x % i == 0 {
            return false
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    // modified instructions - algorithm increments h register when b is not prime
    fn input<'a>() -> &'a str {
        "set b 84
set c b
mul b 100
sub b -100000
set c b
sub c -17000
pri g b
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -6"
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 903);
    }
}
