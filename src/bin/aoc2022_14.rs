use crate::coordinate::{BoundingBox, Vector2D};

const N: usize = 1000;
const POUR_X: usize = 500;
const POUR_Y: usize = 0;

fn main() {
    let mut input = String::new();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                buffer.chars().for_each(|c| input.push(c));
            }
            Err(e) => panic!("{}", e),
        }
    }
    let rock_paths = parser::vector_2d_paths(&input).unwrap().1;
    let mut abyss_cave =
        cave::Cave::from_rock_paths(&rock_paths, coordinate::Vector2D::new(POUR_X, POUR_Y));
    let mut ans1 = 0;
    while abyss_cave.advance() {
        ans1 += 1;
    }
    println!("Part One: {}", ans1);

    let mut floor_cave =
        cave::Cave::from_rock_paths(&rock_paths, coordinate::Vector2D::new(POUR_X, POUR_Y));
    let bounding_box = rock_paths
        .iter()
        .flatten()
        .collect::<Vec<_>>()
        .get_bounding_box();
    floor_cave.add_rock_path(&[
        Vector2D::new(0, bounding_box.1.y + 2),
        Vector2D::new(N - 1, bounding_box.1.y + 2),
    ]);
    let mut ans2 = 1;
    while floor_cave.advance() {
        ans2 += 1;
    }
    println!("Part Two: {}", ans2);
}

mod cave {
    use super::{coordinate::Vector2D, N};
    #[derive(Clone, Copy)]
    enum CellState {
        Air,
        Rock,
        RestingSand,
        FlowingSand,
    }
    impl TryInto<char> for CellState {
        type Error = &'static str;

        fn try_into(self) -> Result<char, Self::Error> {
            let c = match self {
                CellState::Air => '.',
                CellState::Rock => '#',
                CellState::RestingSand => 'o',
                CellState::FlowingSand => '~',
            };
            Ok(c)
        }
    }
    impl CellState {
        fn is_stational(&self) -> bool {
            matches!(self, CellState::Rock | CellState::RestingSand)
        }
    }

    pub struct Cave {
        cells: Vec<Vec<CellState>>,
        pour_from: Vector2D,
    }
    impl Cave {
        pub fn from_rock_paths(rock_paths: &[Vec<Vector2D>], pour_from: Vector2D) -> Self {
            let cells = vec![vec![CellState::Air; N]; N];
            let mut result = Self { cells, pour_from };
            for path in rock_paths.iter() {
                result.add_rock_path(path);
            }
            result
        }
        pub fn add_rock_path(&mut self, rock_path: &[Vector2D]) {
            for line in rock_path.windows(2) {
                let mut current = line[0];
                let end = line[1];
                let dv = end.sig_vec(&current);
                loop {
                    if let Ok(cell) = self.get_mut_cell(current) {
                        *cell = CellState::Rock;
                    }
                    current += dv;
                    if current == end + dv {
                        break;
                    }
                }
            }
        }
        fn out_of_bounds(&self, at: Vector2D) -> bool {
            at.y >= self.cells.len() || at.x >= self.cells[0].len()
        }
        fn get_cell(&self, at: Vector2D) -> Result<&CellState, &'static str> {
            if self.out_of_bounds(at) {
                Err("Out of bounds")
            } else {
                Ok(&self.cells[at.y][at.x])
            }
        }
        fn get_mut_cell(&mut self, at: Vector2D) -> Result<&mut CellState, &'static str> {
            if self.out_of_bounds(at) {
                Err("Out of bounds")
            } else {
                Ok(&mut self.cells[at.y][at.x])
            }
        }
        fn try_flow(&mut self, sand_from: Vector2D, try_at: Vector2D) -> Result<(), &'static str> {
            let is_stational = self.get_cell(try_at)?.is_stational();
            if is_stational {
                Err("Stational")
            } else {
                *self.get_mut_cell(sand_from)? = CellState::Air;
                *self.get_mut_cell(try_at)? = CellState::FlowingSand;
                Ok(())
            }
        }
        pub fn advance(&mut self) -> bool {
            let mut sand_from = self.pour_from;
            loop {
                let mut next_sand = sand_from;
                let below = sand_from + Vector2D::new(0, 1);
                if self.out_of_bounds(below) {
                    return false;
                }
                let below_left = sand_from + Vector2D::new(!0, 1);
                let below_right = sand_from + Vector2D::new(1, 1);
                let try_ats = vec![below, below_left, below_right];
                for try_at in try_ats {
                    if self.try_flow(sand_from, try_at).is_err() {
                        continue;
                    }
                    next_sand = try_at;
                    break;
                }
                if next_sand == sand_from {
                    *self.get_mut_cell(sand_from).unwrap() = CellState::RestingSand;
                    return next_sand != self.pour_from;
                }
                sand_from = next_sand;
            }
        }
    }
    impl std::fmt::Debug for Cave {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut result = vec![];
            for i in 0..self.cells.len() {
                let row = (0..self.cells[0].len())
                    .map(|j| self.cells[i][j].try_into().unwrap())
                    .collect::<Vec<char>>();
                result.push(row.iter().collect::<String>());
            }
            let result = result.join("\n");
            write!(f, "\n{}", result)
        }
    }
}

mod coordinate {
    use std::ops::{Add, AddAssign};

    #[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
    pub struct Vector2D {
        pub x: usize,
        pub y: usize,
    }
    impl Vector2D {
        pub fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }
        pub fn sig_vec(&self, other: &Self) -> Vector2D {
            let dx = signum(self.x, other.x);
            let dy = signum(self.y, other.y);
            Self::new(dx, dy)
        }
    }
    impl AddAssign for Vector2D {
        fn add_assign(&mut self, rhs: Self) {
            *self = Self {
                x: self.x.wrapping_add(rhs.x),
                y: self.y.wrapping_add(rhs.y),
            };
        }
    }
    impl Add for Vector2D {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y))
        }
    }
    fn signum(a: usize, b: usize) -> usize {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => !0,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
    pub trait BoundingBox {
        fn get_bounding_box(&self) -> (Vector2D, Vector2D);
    }
    impl BoundingBox for Vec<&Vector2D> {
        fn get_bounding_box(&self) -> (Vector2D, Vector2D) {
            let max_x = self.iter().map(|v| v.x).max().unwrap();
            let max_y = self.iter().map(|v| v.y).max().unwrap();
            let min_x = self.iter().map(|v| v.x).min().unwrap();
            let min_y = self.iter().map(|v| v.y).min().unwrap();
            (Vector2D::new(min_x, min_y), Vector2D::new(max_x, max_y))
        }
    }
}

mod parser {
    use super::coordinate::Vector2D;
    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::{preceded, tuple},
        IResult,
    };

    fn parsed_usize(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }
    fn vector_2d(input: &str) -> IResult<&str, Vector2D> {
        map(
            tuple((parsed_usize, preceded(tag(","), parsed_usize))),
            |(x, y)| Vector2D { x, y },
        )(input)
    }
    fn vector_2d_path(input: &str) -> IResult<&str, Vec<Vector2D>> {
        separated_list0(tag(" -> "), vector_2d)(input)
    }
    pub fn vector_2d_paths(input: &str) -> IResult<&str, Vec<Vec<Vector2D>>> {
        separated_list0(tag("\n"), vector_2d_path)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_vector_2d() {
            assert_eq!(vector_2d("1,2"), Ok(("", Vector2D { x: 1, y: 2 })));
        }
        #[test]
        fn test_vector_2d_path() {
            assert_eq!(
                vector_2d_path("1,2 -> 3,4 -> 5,6"),
                Ok((
                    "",
                    vec![
                        Vector2D { x: 1, y: 2 },
                        Vector2D { x: 3, y: 4 },
                        Vector2D { x: 5, y: 6 }
                    ]
                ))
            );
        }
        #[test]
        fn test_vector_2d_paths() {
            assert_eq!(
                vector_2d_paths("1,2 -> 3,4 -> 5,6\n7,8 -> 9,10\n"),
                Ok((
                    "",
                    vec![
                        vec![
                            Vector2D { x: 1, y: 2 },
                            Vector2D { x: 3, y: 4 },
                            Vector2D { x: 5, y: 6 }
                        ],
                        vec![Vector2D { x: 7, y: 8 }, Vector2D { x: 9, y: 10 },],
                        vec![]
                    ]
                ))
            );
        }
    }
}
