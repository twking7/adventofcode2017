use std::collections::HashSet;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ASCII;

fn part1(s: &str) -> usize {
    let key_rows: Vec<String> = (0..128).map(|n| format!("{}-{}", s, n)).collect();
    let mut hashes = Vec::new();

    for key_row in key_rows {
        hashes.push(knot_hash(&key_row));
    }

    hashes.iter()
        .map(|hash| hash.iter().map(|n| format!("{:b}", n)).collect())
        .collect::<Vec<String>>()
        .join("")
        .matches("1")
        .count()
}

fn part2(s: &str) -> usize {
    let key_rows: Vec<String> = (0..128).map(|n| format!("{}-{}", s, n)).collect();
    let mut groups = 0;
    let mut hashes = Vec::new();

    for key_row in key_rows {
        hashes.push(knot_hash(&key_row));
    }

    let bit_rows = hashes.iter()
        .map(|hash| hash.iter().map(|n| format!("{:08b}", n)).collect())
        .collect::<Vec<String>>()
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
        .collect::<Vec<Vec<usize>>>();

    let mut matrix = vec![vec![0; 128]; bit_rows.len()];

    for (i, bit_row) in bit_rows.iter().enumerate() {
        for (j, bit) in bit_row.iter().enumerate() {
            if matrix[i][j] == 0 && *bit == 1 {
                let mut visited = HashSet::new();
                match linked_group(i, j, &bit_rows, &mut matrix, &mut visited) {
                    Some(group) => { matrix[i][j] = group; },
                    None => {
                        groups += 1;
                        matrix[i][j] = groups;
                    }
                }
            }
        }
    }

    groups
}

fn linked_group(i: usize, j: usize, bits: &Vec<Vec<usize>>, matrix: &Vec<Vec<usize>>, visited: &mut HashSet<(usize, usize)>) -> Option<usize> {
    if visited.contains(&(i, j)) {
        return None;
    }

    visited.insert((i, j));

    if bits[i][j] == 0 {
        return None;
    }

    if matrix[i][j] > 0 {
        return Some(matrix[i][j]);
    }

    let mut left = None;
    if j > 0 {
        left = linked_group(i, j - 1, bits, matrix, visited);
    }

    let mut up = None;
    if i > 0 {
        up = linked_group(i - 1, j, bits, matrix, visited);
    }

    let mut right = None;
    if j < 127 {
        right = linked_group(i, j + 1, bits, matrix, visited);
    }

    let mut down = None;
    if i < 127 {
        down = linked_group(i + 1, j, bits, matrix, visited);
    }

    vec![left, up, right, down]
        .into_iter()
        .filter(|maybe_group| maybe_group.is_some())
        .map(|maybe_group| maybe_group.unwrap())
        .min()
}

fn knot_hash(input: &str) -> Vec<usize> {
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
        .collect()
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

    fn test_part1() {
        assert_eq!(part1("flqrgnkx"), 8108);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("flqrgnkx"), 1242);
    }
}
