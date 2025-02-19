use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::direction::Direction::*;
use ahash::AHashMap;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::fmt::{Debug, Display, Formatter};
use std::io::Read;
use std::iter::once;

pub struct Day21<'a> {
    codes: Vec<&'a str>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum DirKey {
    Dir(Direction),
    A,
}
use DirKey::*;

impl PartialOrd<Self> for DirKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Dir(d), Dir(o)) => d.ordinal().cmp(&o.ordinal()),
            (Dir(_), A) => Less,
            (A, Dir(_)) => Greater,
            (A, A) => Equal,
        }
    }
}

#[derive(Copy, Clone)]
struct Move {
    h: i8,
    v: i8,
}

impl Move {
    fn new(h: i8, v: i8) -> Self {
        Self { h, v }
    }

    fn opposite(&self) -> Self {
        Self { h: -self.h, v: -self.v }
    }

    fn apply(n: i8, negative: Direction, positive: Direction, r: &mut Vec<Direction>) {
        match n.cmp(&0) {
            Greater => {
                for _ in 0..n {
                    r.push(positive);
                }
            }
            Less => {
                for _ in 0..n.unsigned_abs() {
                    r.push(negative);
                }
            }
            Equal => {}
        }
    }

    fn directions(&self, vertical_first: bool) -> Vec<Direction> {
        let mut r = Vec::with_capacity(5);

        if vertical_first {
            Self::apply(self.v, Up, Down, &mut r);
            Self::apply(self.h, Left, Right, &mut r);
        } else {
            Self::apply(self.h, Left, Right, &mut r);
            Self::apply(self.v, Up, Down, &mut r);
        }

        r
    }

    fn value(&self) -> usize {
        (self.h.unsigned_abs() + self.v.unsigned_abs()) as usize
    }
}

fn numpad_move(f: u8, t: u8) -> Move {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    match f.cmp(&t) {
        Less => {}
        Equal => return Move::new(0, 0),
        Greater => return numpad_move(t, f).opposite(),
    }

    let fh = match f {
        0 => 1,
        10 => 2,
        _ => (f as i8 - 1) % 3,
    };
    let th = if t == 10 { 2 } else { (t as i8 - 1) % 3 };

    let fv = match f {
        0 | 10 => 1,
        _ => -(f as i8 - 1) / 3,
    };
    let tv = if t == 10 { 1 } else { -(t as i8 - 1) / 3 };

    Move::new(th - fh, tv - fv)
}

fn dirpad_move(f: DirKey, t: DirKey) -> Move {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    match f.cmp(&t) {
        Less => match (f, t) {
            (Dir(Up), Dir(Right)) => Move::new(1, 1),
            (Dir(Up), Dir(Down)) => Move::new(0, 1),
            (Dir(Up), Dir(Left)) => Move::new(-1, 1),
            (Dir(Up), A) => Move::new(1, 0),
            (Dir(Right), Dir(Down)) => Move::new(-1, 0),
            (Dir(Right), Dir(Left)) => Move::new(-2, 0),
            (Dir(Right), A) => Move::new(0, -1),
            (Dir(Down), Dir(Left)) => Move::new(-1, 0),
            (Dir(Down), A) => Move::new(1, -1),
            (Dir(Left), A) => Move::new(2, -1),
            _ => unreachable!(),
        },
        Equal => Move::new(0, 0),
        Greater => dirpad_move(t, f).opposite(),
    }
}

type Cache = AHashMap<(DirKey, DirKey, u8), usize>;

fn min_cost<K>(from: K, cost: Move, depth: u8, oob_check: fn(K, &[Direction]) -> bool, cache: &mut Cache) -> usize
where
    K: Copy,
{
    [true, false]
        .into_iter()
        .map(|vertical_first| cost.directions(vertical_first))
        .filter(|directions| !oob_check(from, &directions))
        .map(|directions| {
            directions
                .into_iter()
                .map(Dir)
                .chain(once(A))
                .scan(A, |position, dir| {
                    let cost = dir_cost(*position, dir, depth - 1, cache);
                    *position = dir;
                    Some(cost)
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn dir_cost(from: DirKey, to: DirKey, depth: u8, cache: &mut Cache) -> usize {
    if from == to {
        return 1;
    }

    if let Some(&cost) = cache.get(&(from, to, depth)) {
        return cost;
    }

    let cost = dirpad_move(from, to);
    let result = if depth == 0 {
        cost.value() + 1
    } else {
        min_cost(
            from,
            cost,
            depth,
            |from, directions| match from {
                Dir(Left) => directions.starts_with(&[Up]),
                Dir(Up) => directions.starts_with(&[Left]),
                A => directions.starts_with(&[Left, Left]),
                _ => false,
            },
            cache,
        )
    };
    cache.insert((from, to, depth), result);
    result
}

fn num_cost(from: u8, to: u8, depth: u8, cache: &mut Cache) -> usize {
    if from == to {
        return 1;
    }

    let cost = numpad_move(from, to);
    if depth == 0 {
        cost.value() + 1
    } else {
        min_cost(
            from,
            cost,
            depth,
            |from, directions| match from {
                0 => directions.starts_with(&[Left]),
                1 => directions.starts_with(&[Down]),
                4 => directions.starts_with(&[Down, Down]),
                7 => directions.starts_with(&[Down, Down, Down]),
                10 => directions.starts_with(&[Left, Left]),
                _ => false,
            },
            cache,
        )
    }
}

impl Day21<'_> {
    fn min_move_length(&self, depth: u8) -> usize {
        let mut cache = AHashMap::new();
        self.codes
            .iter()
            .map(|&code| {
                code.bytes()
                    .map(|k| if k == b'A' { 10 } else { k - b'0' })
                    .scan(10, |position, num| {
                        let cost = num_cost(*position, num, depth, &mut cache);
                        *position = num;
                        Some(cost)
                    })
                    .sum::<usize>()
                    * code.strip_suffix('A').unwrap().parse::<usize>().unwrap()
            })
            .sum()
    }
}

impl<'a> Day<'a> for Day21<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            codes: input.lines().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.min_move_length(2)
    }

    fn part_2(&self) -> Self::T2 {
        self.min_move_length(25)
    }
}
