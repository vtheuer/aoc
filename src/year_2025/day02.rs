use crate::day::Day;
use num::Integer;

pub struct Day02 {
    ranges: Vec<(usize, usize)>,
}

impl Day<'_> for Day02 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            ranges: input
                .lines()
                .next()
                .unwrap()
                .split(',')
                .filter_map(|l| l.split_once('-'))
                .filter_map(|(f, t)| Some((f.parse().ok()?, t.parse().ok()?)))
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.ranges
            .iter()
            .map(|&(f, t)| {
                (f..=t)
                    .filter(|&n| {
                        let len = n.ilog10() + 1;
                        let h = 10usize.pow(len / 2);
                        len.is_even() && n / h == n % h
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        self.ranges
            .iter()
            .map(|&(f, t)| {
                (f..=t)
                    .filter(|&n| {
                        let len = (n.ilog10() + 1) as usize;
                        (2usize..=len).filter(|&times| len.is_multiple_of(times)).any(|times| {
                            let part_len = len / times;
                            let p = 10usize.pow(part_len as u32);
                            let first = n / 10usize.pow((part_len * (times - 1)) as u32) % p;
                            (1..times).all(|t| first == n / 10usize.pow((part_len * (times - 1 - t)) as u32) % p)
                        })
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}
