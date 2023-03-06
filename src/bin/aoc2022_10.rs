use itertools::Itertools;

fn main() {
    let instructions = parser::parse(&scanner::scan());
    let mut crt = vec![vec![false; 40]; 6];
    let mut register_x = 1i32;

    let mut ans1 = 0;
    for (i, v) in instructions.iter().enumerate() {
        if i % 40 == 19 {
            ans1 += (i + 1) as i32 * register_x;
        }
        let r = i / 40;
        let c = i % 40;
        if (register_x - 1..register_x + 2).contains(&(c as i32)) {
            crt[r][c] = true;
        }
        register_x += v;
    }
    let ans2 = crt
        .iter()
        .map(|line| line.iter().map(|&b| ['.', '#'][b as usize]).join(""))
        .join("\n");
    println!("Part One: {}", ans1);
    println!("Part Two:\n{}", ans2);
}

mod scanner {
    use std::io::Read;

    pub fn scan() -> String {
        let mut buffer = vec![];
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_end(&mut buffer).unwrap();
        match std::str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
        .to_string()
    }
}
mod parser {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::i32 as nom_i32, combinator::map,
        multi::separated_list0, sequence::preceded, IResult,
    };

    fn noop(input: &str) -> IResult<&str, Vec<i32>> {
        map(tag("noop"), |_: &str| vec![0])(input)
    }
    fn addx(input: &str) -> IResult<&str, Vec<i32>> {
        map(preceded(tag("addx "), nom_i32), |v| vec![0, v])(input)
    }
    fn instruction(input: &str) -> IResult<&str, Vec<i32>> {
        alt((noop, addx))(input)
    }
    fn program(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
        separated_list0(tag("\n"), instruction)(input)
    }
    pub fn parse(input: &str) -> Vec<i32> {
        program(input).unwrap().1.into_iter().flatten().collect()
    }
}
