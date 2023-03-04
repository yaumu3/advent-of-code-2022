use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Debug, Clone)]
pub enum Item {
    V(usize),
    L(Vec<Item>),
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::V(a), Item::V(b)) => a.cmp(b),
            (Item::L(a), Item::L(b)) => a.iter().cmp(b),
            (Item::V(a), Item::L(_)) => Item::L(vec![Item::V(*a)]).cmp(other),
            (Item::L(_), Item::V(_)) => other.cmp(self).reverse(),
        }
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Item {}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
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

    let pairs = parser::item_pairs(&input).unwrap().1;
    let ans1 = pairs
        .iter()
        .enumerate()
        .filter(|(_i, p)| p.0 < p.1)
        .fold(0, |acc, (i, _p)| acc + i + 1);
    println!("Part One: {}", ans1);

    let mut signals: BTreeSet<_> = pairs.into_iter().flat_map(|(a, b)| vec![a, b]).collect();
    let divider_packet_1 = Item::L(vec![Item::L(vec![Item::V(2)])]);
    let divider_packet_2 = Item::L(vec![Item::L(vec![Item::V(6)])]);
    signals.insert(divider_packet_1.clone());
    signals.insert(divider_packet_2.clone());
    let ans2 =
        signals.range(..=divider_packet_1).count() * signals.range(..=divider_packet_2).count();
    println!("Part Two: {}", ans2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_value_value() {
        let a = Item::V(4);
        let b = Item::V(4);
        assert!(a == b);
    }
    #[test]
    fn ne_value_value() {
        let a = Item::V(4);
        let b = Item::V(5);
        assert!(a < b);
    }
    #[test]
    fn eq_list_list() {
        let a = Item::L(vec![Item::L(vec![Item::V(4), Item::V(5)]), Item::V(5)]);
        let b = Item::L(vec![Item::L(vec![Item::V(4), Item::V(5)]), Item::V(5)]);
        assert!(a == b);
    }
    #[test]
    fn ne_list_list_lack_of_item() {
        let a = Item::L(vec![Item::L(vec![Item::V(4), Item::V(5)]), Item::V(5)]);
        let b = Item::L(vec![Item::L(vec![Item::V(4), Item::V(5)])]);
        assert!(a > b);
    }
    #[test]
    fn ne_list_list_nest() {
        let a = Item::L(vec![Item::L(vec![Item::L(vec![])])]);
        let b = Item::L(vec![Item::L(vec![])]);
        assert!(a > b);
    }
    #[test]
    fn ne_list_list_ne_value() {
        let a = Item::L(vec![Item::V(4), Item::V(5)]);
        let b = Item::L(vec![Item::V(6), Item::V(5)]);
        assert!(a < b);
    }
    #[test]
    fn eq_value_list() {
        let a = Item::V(4);
        let b = Item::L(vec![Item::V(4)]);
        assert!(a == b);
    }
    #[test]
    fn ne_value_list_value_greater() {
        let a = Item::V(5);
        let b = Item::L(vec![Item::V(4)]);
        assert!(a > b);
    }
    #[test]
    fn ne_value_list_list_greater() {
        let a = Item::V(4);
        let b = Item::L(vec![Item::V(5)]);
        assert!(a < b);
    }
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::{preceded, terminated, tuple},
        IResult,
    };

    fn value(input: &str) -> IResult<&str, Item> {
        map(map_res(digit1, |s: &str| s.parse::<usize>()), Item::V)(input)
    }
    fn item(input: &str) -> IResult<&str, Item> {
        map(
            preceded(
                tag("["),
                terminated(separated_list0(tag(","), alt((value, item))), tag("]")),
            ),
            Item::L,
        )(input)
    }
    fn item_pair(input: &str) -> IResult<&str, (Item, Item)> {
        tuple((item, preceded(tag("\n"), item)))(input)
    }
    pub fn item_pairs(input: &str) -> IResult<&str, Vec<(Item, Item)>> {
        separated_list0(tag("\n\n"), item_pair)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn value_parser() {
            assert_eq!(value("123"), Ok(("", Item::V(123))));
        }
        #[test]
        fn item_parser() {
            assert_eq!(
                item("[1,2,3]"),
                Ok(("", Item::L(vec![Item::V(1), Item::V(2), Item::V(3)])))
            );
            assert_eq!(
                item("[1,2,[1,[],2],3]"),
                Ok((
                    "",
                    Item::L(vec![
                        Item::V(1),
                        Item::V(2),
                        Item::L(vec![Item::V(1), Item::L(vec![]), Item::V(2)]),
                        Item::V(3)
                    ])
                ))
            );
            assert_eq!(
                item("[[[]]]"),
                Ok(("", Item::L(vec![Item::L(vec![Item::L(vec![])]),])))
            );
        }
        #[test]
        fn item_pair_parser() {
            assert_eq!(
                item_pair("[1]\n[2]"),
                Ok(("", (Item::L(vec![Item::V(1)]), Item::L(vec![Item::V(2)]))))
            );
        }
        #[test]
        fn item_pairs_parser() {
            assert_eq!(
                item_pairs("[]\n[]\n\n[]\n[]"),
                Ok((
                    "",
                    vec![
                        (Item::L(vec![]), Item::L(vec![])),
                        (Item::L(vec![]), Item::L(vec![]))
                    ]
                ))
            );
        }
    }
}
