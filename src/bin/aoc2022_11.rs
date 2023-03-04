use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
}
impl FromStr for Operator {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mul),
            _ => Err("error"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Old,
    Fixed(usize),
}
impl FromStr for Operand {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            _ => match s.parse() {
                Ok(n) => Ok(Operand::Fixed(n)),
                _ => Err("error"),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    operand_left: Operand,
    operand_right: Operand,
}
impl Operation {
    fn operate(&self, old: usize) -> usize {
        let resolve_value = |o: &Operand| match *o {
            Operand::Old => old,
            Operand::Fixed(n) => n,
        };

        let left = resolve_value(&self.operand_left);
        let right = resolve_value(&self.operand_right);
        match self.operator {
            Operator::Add => left + right,
            Operator::Mul => left * right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    inspect_count: usize,
    targets: Vec<usize>,
    operation: Operation,
    test_mod: usize,
    pass_true: usize,
    pass_false: usize,
}
impl Monkey {
    fn push(&mut self, value: usize) {
        self.targets.push(value)
    }
    fn inspect(&mut self, modulo: usize, decay_factor: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];
        while let Some(t) = self.targets.pop() {
            self.inspect_count += 1;
            let nt = self.operation.operate(t) / decay_factor % modulo;
            result.push(match nt % self.test_mod {
                0 => (self.pass_true, nt),
                _ => (self.pass_false, nt),
            });
        }
        result
    }
}

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
    let mut monkeys = parser::monkeys(&input).unwrap().1;

    let calc_level_of_monkey_business =
        |monkeys: &mut Vec<Monkey>, rounds: usize, decay_factor: usize| {
            let test_mods_lcm = monkeys.iter().map(|m| m.test_mod).fold(1, lcm);
            for _ in 0..rounds {
                for i in 0..monkeys.len() {
                    for (to_i, value) in monkeys[i].inspect(test_mods_lcm, decay_factor) {
                        monkeys[to_i].push(value);
                    }
                }
            }
            let mut result: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
            result.sort_unstable_by(|a, b| b.cmp(a));
            assert!(result.len() >= 2);
            result[..2].iter().product::<usize>()
        };

    let ans1 = calc_level_of_monkey_business(&mut monkeys.clone(), 20, 3);
    println!("Part One: {}", ans1);
    let ans2 = calc_level_of_monkey_business(&mut monkeys, 10000, 1);
    println!("Part Two: {}", ans2);
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + PartialEq + std::ops::Rem<Output = T> + std::ops::Add<Output = T>,
{
    if b == b + b {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + PartialEq
        + std::ops::Rem<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    a / gcd(a, b) * b
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        character::{complete::digit1, streaming::one_of},
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::{preceded, terminated, tuple},
        IResult,
    };

    fn sp(i: &str) -> IResult<&str, &str> {
        let chars = " \t\r\n";
        take_while(move |c| chars.contains(c))(i)
    }
    fn num(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }
    fn monkey_index(input: &str) -> IResult<&str, usize> {
        preceded(tag("Monkey "), terminated(num, tag(":")))(input)
    }
    fn targets(input: &str) -> IResult<&str, Vec<usize>> {
        preceded(tag("Starting items: "), separated_list0(tag(", "), num))(input)
    }
    fn operand(input: &str) -> IResult<&str, Operand> {
        map_res(alt((digit1, tag("old"))), Operand::from_str)(input)
    }
    fn operator(input: &str) -> IResult<&str, Operator> {
        map_res(preceded(sp, terminated(one_of("*+"), sp)), |c| {
            Operator::from_str(&c.to_string())
        })(input)
    }
    fn operation(input: &str) -> IResult<&str, Operation> {
        map(
            preceded(
                tag("Operation: new = "),
                tuple((operand, operator, operand)),
            ),
            |(operand_left, operator, operand_right)| Operation {
                operand_left,
                operator,
                operand_right,
            },
        )(input)
    }
    fn test_mod(input: &str) -> IResult<&str, usize> {
        preceded(tag("Test: divisible by "), num)(input)
    }
    fn throw_true(input: &str) -> IResult<&str, usize> {
        preceded(tag("If true: throw to monkey "), num)(input)
    }
    fn throw_false(input: &str) -> IResult<&str, usize> {
        preceded(tag("If false: throw to monkey "), num)(input)
    }
    fn monkey(input: &str) -> IResult<&str, Monkey> {
        map(
            tuple((
                monkey_index,
                preceded(sp, targets),
                preceded(sp, operation),
                preceded(sp, test_mod),
                preceded(sp, throw_true),
                preceded(sp, throw_false),
            )),
            |(_, targets, operation, test_mod, pass_true, pass_false)| Monkey {
                inspect_count: 0,
                targets,
                operation,
                test_mod,
                pass_true,
                pass_false,
            },
        )(input)
    }
    pub fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
        separated_list0(tag("\n\n"), monkey)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sp() {
            assert_eq!(sp(" \ts"), Ok(("s", " \t")));
        }
        #[test]
        fn test_num() {
            assert_eq!(num("1"), Ok(("", 1)));
        }
    }
}
