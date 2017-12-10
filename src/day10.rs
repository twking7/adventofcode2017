use encoding::{Encoding, EncoderTrap};
use encoding::all::ASCII;

fn part1(input: &str, mut nums: Vec<usize>) -> usize {
    let steps = str_to_ints(input);
    let len = nums.len();
    let mut ptr = 0;
    let mut skip = 0;

    for step in steps {
        let mut nums_to_rev = Vec::new();

        for i in ptr..(ptr + step) {
            nums_to_rev.push(nums[i % len]);
        }
        nums_to_rev.reverse();

        for i in ptr..(ptr + step) {
            nums[i % len] = nums_to_rev[i - ptr];
        }
        ptr += (step + skip) % len;
        skip += 1;
    }

    nums[0] * nums[1]
}

fn part2(input: &str) -> String {
    let steps: Vec<usize> = str_to_bytes(input).into_iter().map(|n| n as usize).collect();
    let mut nums: Vec<usize> = (0..256).collect();
    let len = nums.len();
    let mut ptr = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for step in &steps {
            let mut nums_to_rev = Vec::new();

            for i in ptr..(ptr + step) {
                nums_to_rev.push(nums[i % len]);
            }
            nums_to_rev.reverse();

            for i in ptr..(ptr + step) {
                nums[i % len] = nums_to_rev[i - ptr];
            }
            ptr += (step + skip) % len;
            skip += 1;
        }
    }

    nums.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &n| acc ^ n))
        .map(|n| format!("{:02x}", n))
        .collect::<Vec<String>>().join("")
}

fn str_to_ints(s: &str) -> Vec<usize> {
    s.split(',').map(|c| c.parse().expect("numerical input")).collect()
}

fn str_to_bytes(s: &str) -> Vec<u8> {
    let mut bytes = ASCII.encode(s, EncoderTrap::Strict).expect("valid ASCII");
    let mut suffix = vec![17, 31, 73, 47, 23];
    bytes.append(&mut suffix);
    bytes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("3,4,1,5", (0..5).collect()), 12);
    }

    #[test]
    fn test_str_to_bytes() {
        assert_eq!(str_to_bytes("1,2,3"), vec![49,44,50,44,51,17,31,73,47,23]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272".to_string());
        assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd".to_string());
        assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d".to_string());
        assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e".to_string());
    }
}
