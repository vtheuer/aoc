use std::clone;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::mem::swap;
use Ordering::{Equal, Greater, Less};

use fnv::FnvHashMap;

use crate::day::Day;

pub struct Day20 {
    numbers: Vec<isize>,
}

impl Day<'_> for Day20 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            numbers: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut shifts = vec![0; self.numbers.len()];

        for (i, &n) in self.numbers.iter().enumerate() {
            let new_i = (i as isize - shifts[i]) as usize;
            match n.cmp(&0) {
                Greater => {
                    let j = new_i + n as usize;
                    let s = shifts[new_i];
                    dbg!(i);
                    dbg!(n);
                    dbg!(new_i);
                    dbg!(j);
                    dbg!(s);
                    for k in new_i..j {
                        shifts[k] = shifts[k + 1] + 1;
                    }
                    shifts[j] = s - n;
                }
                Less => {
                    if new_i as isize + n >= 0 {
                        unimplemented!()
                    } else {
                        let j = (new_i as isize + shifts.len() as isize - 1 + n) as usize;
                        let s = shifts[new_i];
                        dbg!(i);
                        dbg!(n);
                        dbg!(new_i);
                        dbg!(j);
                        dbg!(s);
                        for k in new_i..j {
                            shifts[k] = shifts[k + 1] + 1;
                        }
                        shifts[j] = s - (j - new_i) as isize;
                    }
                }
                _ => {}
            }
            dbg!(&shifts);
            dbg!(self.apply(&shifts));
        }

        0
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}

impl Day20 {
    fn apply(&self, shifts: &[isize]) -> Vec<isize> {
        shifts
            .iter()
            .enumerate()
            .map(|(i, &shift)| self.numbers[(i as isize + shift) as usize])
            .collect()
    }
}