use std::ops::AddAssign;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
struct Vector(isize, isize, isize);

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.0;
        self.1 = self.1 + other.1;
        self.2 = self.2 + other.2;
    }
}

impl Vector {
    fn norm(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}

#[derive(Debug, PartialEq)]
struct Particle {
    name: usize,
    p: Vector,
    v: Vector,
    a: Vector,
}

impl Particle {
    fn step(&mut self) {
        self.v += self.a.clone();
        self.p += self.v.clone();
    }
}

fn part1(s: &str) -> usize {
    let mut particles = str_to_particles(s);

    for _ in 0..100_000 {
        for mut particle in particles.iter_mut() {
            particle.step();
        }
    }

    particles.sort_by(|a, b| a.p.norm().cmp(&b.p.norm()));
    particles.iter().nth(0).unwrap().name
}

fn part2(s: &str) -> usize {
    let mut particles = str_to_particles(s);

    for _ in 0..1_000 {
        remove_collisions(&mut particles);

        for mut particle in particles.iter_mut() {
            particle.step();
        }
    }

    particles.len()
}

fn remove_collisions(particles: &mut Vec<Particle>) {
    let mut removals = HashSet::new();

    for i in 0..particles.len() {
        for j in 0..particles.len() {
            if i != j && particles[i].p == particles[j].p {
                removals.insert(particles[j].name.clone());
                removals.insert(particles[i].name.clone());
            }
        }
    }

    particles.retain(|particle| !removals.contains(&particle.name));
}

fn str_to_particles(lines: &str) -> Vec<Particle> {
    let vectors = lines.lines().map(|line| {
        line
            .split_whitespace()
            .map(|s| &s[3..(s.len() - 1)])
            .map(|v| {
                v.split(',').map(|spl| {
                    spl.replace('>', "").parse().unwrap()
                }).collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>()
            .into_iter()
            .map(|p| Vector(p[0], p[1], p[2]))
            .collect()
    }).collect::<Vec<Vec<Vector>>>();


    vectors
        .into_iter()
        .enumerate()
        .map(|(i, v)| Particle { name: i, p: v[0].clone(), v: v[1].clone(), a: v[2].clone() })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\np=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

        assert_eq!(part2(input), 1);
    }
}
