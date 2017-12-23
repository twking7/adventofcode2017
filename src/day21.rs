use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Matrix {
    m: Vec<Vec<char>>,
}

impl Matrix {
    fn new(size: usize) -> Self {
        Self { m: vec![vec!['.'; size]; size] }
    }

    fn from_subs(subs: &[Vec<Matrix>]) -> Self {
        let sub_size = subs[0][0].size();
        let new_size = subs.len() * sub_size;
        let mut v = vec![String::new(); new_size];

        for i in 0..new_size {
            for j in 0..new_size {
                let mat = &subs[i / sub_size][j / sub_size];
                let char_str = mat.m[i % sub_size][j % sub_size].to_string();
                v[i].push_str(&char_str);
            }
        }

        Self::from(v.join("/").as_str())
    }

    fn size(&self) -> usize {
        self.m.len()
    }

    fn flip_y(&self) -> Self {
        let mut m = self.m.clone();

        for mut line in &mut m {
            line.reverse();
        }

        Self { m: m }
    }

    fn flip_x(&self) -> Self {
        let mut m = self.m.clone();
        m.reverse();
        Self { m: m }
    }

    fn rotate(&self) -> Self {
        let n = self.size();
        let mut t = Matrix::new(n);

        for i in 0..n {
            for j in 0..n {
                t.m[i][j] = self.m[j][i];
            }
        }

        t.flip_x()
    }

    fn to_subs(&self, size: usize) -> Vec<Vec<Matrix>> {
        let subsize = self.size() / size;
        let mut out = vec![vec![Matrix::new(size); subsize]; subsize];

        for (i, mut row) in out.iter_mut().enumerate() {
            for (j, mut col) in row.iter_mut().enumerate() {
                for k in 0..size {
                    for l in 0..size {
                        col.m[k][l] = self.m[(i * size) + k][(j * size) + l];
                    }
                }
            }
        }

        out
    }
}

impl<'a> From<&'a str> for Matrix {
    fn from(s: &'a str) -> Self {
        let m = s.split('/')
            .map(|s| s.chars().collect())
            .collect();
        Self { m: m }
    }
}

impl ToString for Matrix {
    fn to_string(&self) -> String {
        self.m
            .iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("/")
    }
}

fn part1(s: &str, cycles: usize) -> usize {
    let enhancements = make_enhancements(s);
    let mut matrix = Matrix::from(".#./..#/###");

    for _ in 0..cycles {
        if matrix.size() == 3 {
            let pattern = match_pattern(&matrix, &enhancements);
            matrix = Matrix::from(pattern.as_str());
        } else if matrix.size() % 2 == 0 || matrix.size() % 3 == 0 {
            let size = if matrix.size() % 2 == 0 { 2 } else { 3 };
            let subs = matrix.to_subs(size);

            let mats = subs
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|sub| Matrix::from(match_pattern(sub, &enhancements).as_str()))
                        .collect::<Vec<Matrix>>()
                }).collect::<Vec<Vec<Matrix>>>();

            matrix = Matrix::from_subs(&mats);
        }
    }

    matrix.to_string().matches('#').count()
}

fn match_pattern(matrix: &Matrix, enhancements: &HashMap<&str, &str>) -> String {
    let r90 = matrix.rotate();
    let r180 = r90.rotate();
    let r270 = r180.rotate();

    let pattern = if let Some(p) = enhancements.get(&matrix.to_string() as &str) {
        p
    } else if let Some(p) = enhancements.get(&matrix.flip_y().to_string() as &str) {
        p
    } else if let Some(p) = enhancements.get(&r90.to_string() as &str) {
        p
    } else if let Some(p) = enhancements.get(&r90.flip_y().to_string() as &str) {
        p
    } else if let Some(p) = enhancements.get(&r180.to_string() as &str) {
        p
    } else if let Some(p) = enhancements.get(&r180.flip_y().to_string() as &str) {
        p
    } else if let Some(p) = enhancements.get(&r270.to_string() as &str) {
        p
    } else if let Some(p) = enhancements.get(&r270.flip_y().to_string() as &str) {
        p
    } else {
        panic!("could not match {}", matrix.to_string());
    };

    pattern.to_string()
}

fn make_enhancements(s: &str) -> HashMap<&str, &str> {
    s.lines()
        .map(|line| line.split(" => ").collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut h, v| {
            h.insert(v[0], v[1]);
            h
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";

        assert_eq!(part1(input, 2), 12);
    }
}
