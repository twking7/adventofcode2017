use std::collections::HashMap;

enum State { A, B, C, D, E, F }

fn part1() -> usize {
    let mut i = 0;
    let mut ptr = 0isize;
    let mut h: HashMap<isize, bool> = HashMap::new();
    let mut state = State::A;

    while i < 12_994_925 {
        let val = *h.get(&ptr).unwrap_or(&false);

        match state {
            State::A => {
                if val {
                    h.insert(ptr, false);
                    ptr -= 1;
                    state = State::F;
                } else {
                    h.insert(ptr, true);
                    ptr += 1;
                    state = State::B;
                }
            },
            State::B => {
                if val {
                    h.insert(ptr, false);
                    ptr += 1;
                    state = State::D;
                } else {
                    h.insert(ptr, false);
                    ptr += 1;
                    state = State::C;
                }
            },
            State::C => {
                if val {
                    h.insert(ptr, true);
                    ptr += 1;
                    state = State::E;
                } else {
                    h.insert(ptr, true);
                    ptr -= 1;
                    state = State::D;
                }
            },
            State::D => {
                if val {
                    h.insert(ptr, false);
                    ptr -= 1;
                    state = State::D;
                } else {
                    h.insert(ptr, false);
                    ptr -= 1;
                    state = State::E;
                }
            },
            State::E => {
                if val {
                    h.insert(ptr, true);
                    ptr += 1;
                    state = State::C;
                } else {
                    h.insert(ptr, false);
                    ptr += 1;
                    state = State::A;
                }
            },
            State::F => {
                if val {
                    h.insert(ptr, true);
                    ptr += 1;
                    state = State::A;
                } else {
                    h.insert(ptr, true);
                    ptr -= 1;
                    state = State::A;
                }
            }
        }

        i += 1;
    }

    h.values().into_iter().filter(|v| **v).count()
}
