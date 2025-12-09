use crate::day::Day;
use std::cmp::max;

pub struct Day03 {
    banks: Vec<Vec<u8>>,
}

impl Day<'_> for Day03 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            banks: input
                .lines()
                .map(|l| l.as_bytes().iter().map(|&b| b - b'0').collect())
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.banks
            .iter()
            .map(|bank| {
                let max_index = bank[0..bank.len() - 1]
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by_key(|(_, n)| *n)
                    .unwrap()
                    .0;
                (bank[max_index] * 10 + bank[max_index + 1..].iter().copied().max().unwrap()) as usize
            })
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.banks
            .iter()
            .map(|bank| {
                (0..12usize)
                    .rev()
                    .scan(None, |prev, pow| {
                        let p = prev.map(|p| p + 1).unwrap_or(0);
                        let max_index = p + bank[p..bank.len() - pow]
                            .iter()
                            .enumerate()
                            .rev()
                            .max_by_key(|(_, n)| *n)
                            .unwrap()
                            .0;
                        *prev = Some(max_index);
                        Some((pow, max_index))
                    })
                    .map(|(pow, i)| bank[i] as usize * 10usize.pow(pow as u32))
                    .sum::<usize>()
            })
            .sum()
    }
}
