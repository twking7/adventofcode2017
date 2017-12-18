use std::collections::HashMap;

struct Program<'a> {
    reg: HashMap<&'a str, isize>,
    lines: Vec<Vec<&'a str>>,
    ptr: isize,
    snd_count: usize,
    term: bool,
}

impl<'a> Program<'a> {
    fn new(s: &'a str, num: isize) -> Self {
        let lines: Vec<Vec<&str>> = s.lines().map(|l| l.split_whitespace().collect()).collect();
        let mut h = HashMap::new();
        h.insert("p", num);

        Self { reg: h, lines: lines, ptr: 0, snd_count: 0, term: false }
    }

    fn run<'b>(&mut self, in_queue: &'b mut Vec<isize>, out_queue: &'b mut Vec<isize>) {
        loop {
            let mut jumped = false;
            let asm = &self.lines[self.ptr as usize];

            let lop = match asm[1].parse::<isize>() {
                Ok(i) => i,
                Err(_) => *self.reg.get(asm[1]).unwrap_or(&0),
            };

            if asm.len() == 2 {
                match asm[0] {
                    "snd" => {
                        out_queue.push(lop);
                        self.snd_count += 1;
                    },
                    "rcv" => {
                        if !in_queue.is_empty() {
                            let n = in_queue.remove(0);
                            self.reg.insert(asm[1], n);
                        } else {
                            return
                        }
                    },
                    _ => panic!("unexpected op: {}", asm[0]),
                }
            } else {
                let rop = match asm[2].parse::<isize>() {
                    Ok(i) => i,
                    Err(_) => *self.reg.get(asm[2]).unwrap_or(&0),
                };

                match asm[0] {
                    "set" => { self.reg.insert(asm[1], rop); },
                    "add" => { self.reg.insert(asm[1], lop + rop); },
                    "mul" => { self.reg.insert(asm[1], lop * rop); },
                    "mod" => { self.reg.insert(asm[1], lop % rop); },
                    "jgz" => {
                        if lop > 0 {
                            jumped = true;
                            self.ptr += rop;
                        }
                    },
                    _ => panic!("unexpected op: {}", asm[0]),
                }
            }

            if !jumped { self.ptr += 1 }
            if self.ptr < 0 || (self.ptr as usize) >= self.lines.len() {
                self.term = true;
                return
            }
        }
    }
}

fn part1(s: &str) -> isize {
    let mut reg: HashMap<&str, isize> = HashMap::new();
    let lines: Vec<&str> = s.lines().collect();
    let mut ptr = 0isize;
    let mut last = 0;

    loop {
        let asm: Vec<&str> = lines[ptr as usize].split_whitespace().collect();
        ptr += 1;

        let lop = match asm[1].parse::<isize>() {
            Ok(i) => i,
            Err(_) => *reg.get(asm[1]).unwrap_or(&0),
        };

        if asm.len() == 2 {
            match asm[0] {
                "snd" => last = lop,
                "rcv" => if lop != 0 { return last },
                _ => panic!("unexpected op: {}", asm[0]),
            }
        } else {
            let rop = match asm[2].parse::<isize>() {
                Ok(i) => i,
                Err(_) => *reg.get(asm[2]).unwrap_or(&0),
            };

            match asm[0] {
                "set" => { reg.insert(asm[1], rop); },
                "add" => { reg.insert(asm[1], lop + rop); },
                "mul" => { reg.insert(asm[1], lop * rop); },
                "mod" => { reg.insert(asm[1], lop % rop); },
                "jgz" => { if lop > 0 { ptr += rop - 1 } },
                _ => panic!("unexpected op: {}", asm[0]),
            }
        }

        if ptr < 0 || (ptr as usize) >= lines.len() { return last }
    }
}

fn part2(s: &str) -> usize {
    let mut prog0 = Program::new(s, 0);
    let mut prog1 = Program::new(s, 1);
    let mut queue0 = Vec::new();
    let mut queue1 = Vec::new();

    while !prog0.term && !prog1.term {
        prog0.run(&mut queue0, &mut queue1);
        prog1.run(&mut queue1, &mut queue0);

        if queue0.is_empty() && queue1.is_empty() { break }
    }

    prog1.snd_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part2() {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        assert_eq!(part2(input), 3);
    }
}
