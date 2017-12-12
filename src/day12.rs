use std::collections::{HashMap, HashSet};

fn part1(s: &str) -> usize {
    let h = make_map(s);
    let mut visited = HashSet::new();

    visit_nodes(0, &h, &mut visited);

    visited.len()
}

fn part2(s: &str) -> usize {
    let h = make_map(s);
    let mut groups = 0;
    let mut visited = HashSet::new();

    for i in 0..h.len() {
        if !visited.contains(&i) {
            groups += 1;
            visit_nodes(i, &h, &mut visited);
        }
    }

    groups
}

fn visit_nodes(curr: usize, map: &HashMap<usize, Vec<usize>>, mut visited: &mut HashSet<usize>) {
    if visited.contains(&curr) { return }
    visited.insert(curr);

    let child_ids = map.get(&curr).unwrap();

    for child_id in child_ids {
        visit_nodes(*child_id, &map, &mut visited);
    }
}

fn make_map(s: &str) -> HashMap<usize, Vec<usize>> {
    let mut h = HashMap::new();

    for line in s.split('\n').collect::<Vec<&str>>() {
        let parts = line.split(" <-> ").collect::<Vec<&str>>();
        let id: usize = parts[0].parse().unwrap();
        let ids: Vec<usize> = parts[1].split(", ").map(|n| n.parse().unwrap()).collect();
        h.insert(id, ids);
    }

    h
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!(part2(input), 2);
    }
}
