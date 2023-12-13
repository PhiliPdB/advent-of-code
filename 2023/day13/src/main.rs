use std::ops::Not;
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Ash, Rock
}

impl Field {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Field::Ash,
            '#' => Field::Rock,
            _ => panic!("Unknown field type: {c}"),
        }
    }
}

impl Not for Field {
    type Output = Field;

    fn not(self) -> Self::Output {
        match self {
            Field::Ash => Field::Rock,
            Field::Rock => Field::Ash,
        }
    }
}

#[derive(Debug)]
pub struct Pattern {
    rows: Vec<Vec<Field>>,
    columns: Vec<Vec<Field>>,
}

impl Pattern {
    pub fn new(rows: Vec<Vec<Field>>) -> Self {
        let columns = (0..rows[0].len())
            .map(|x| rows.iter().map(|row| row[x]).collect())
            .collect();

        Pattern { rows, columns }
    }

    pub fn fix_smudge(&mut self) -> usize {
        let skip_x = self.horizontal_reflection_except(0).unwrap_or(0);
        let skip_y = self.vertical_reflection_except(0).unwrap_or(0);

        for y in 0..self.rows.len() {
            for x in 0..self.columns.len() {
                self.rows[y][x] = !self.rows[y][x];
                self.columns[x][y] = !self.columns[x][y];

                let reflection = self.horizontal_reflection_except(skip_x)
                    .unwrap_or_else(
                        || 100 * self.vertical_reflection_except(skip_y).unwrap_or(0)
                    );
                if reflection != 0 {
                    return reflection;
                }

                self.rows[y][x] = !self.rows[y][x];
                self.columns[x][y] = !self.columns[x][y];
            }
        }

        skip_x + 100 * skip_y
    }

    pub fn reflection(&self) -> usize {
        self.horizontal_reflection_except(0)
            .unwrap_or_else(
                || 100 * self.vertical_reflection_except(0).unwrap_or(0)
            )
    }

    fn horizontal_reflection_except(&self, skip_x: usize) -> Option<usize> {
        for x in 1..self.columns.len() {
            if x == skip_x {
                continue;
            }

            if self.rows.iter().all(|row| Self::reflects(row, x)) {
                return Some(x);
            }
        }

        None
    }

    fn vertical_reflection_except(&self, skip_y: usize) -> Option<usize> {
        for y in 1..self.rows.len() {
            if y == skip_y {
                continue;
            }

            if self.columns.iter().all(|column| Self::reflects(column, y)) {
                return Some(y);
            }
        }

        None
    }

    fn reflects(list: &[Field], index: usize) -> bool {
        list[index..].iter().zip(list[..index].iter().rev())
            .all(|(a, b)| *a == *b)
    }
}


impl FromStr for Pattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| line.chars().map(Field::from_char).collect())
            .collect();

        Ok(Pattern::new(rows))
    }
}

fn main() {
    let mut patterns: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| s.parse::<Pattern>().unwrap())
        .collect();

    let note_summary: usize = patterns.iter()
        .map(|p| p.reflection())
        .sum();
    println!("[Part 1] Note summary: {note_summary}");

    let smudge_fixing: usize = patterns.iter_mut()
        .map(|p| p.fix_smudge())
        .sum();
    println!("[Part 2] Smudge fixing: {smudge_fixing}");
}
