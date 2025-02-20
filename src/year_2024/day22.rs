use crate::day::Day;
use ahash::{AHashMap, AHashSet};
use std::cell::Cell;
use std::collections::BTreeMap;

pub struct Day22 {
    secrets: Vec<usize>,
    all_prices: Cell<Vec<Vec<i8>>>,
}

fn mix(secret: usize, v: usize) -> usize {
    secret ^ v
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn compute(mut secret: usize) -> usize {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));

    secret
}

impl Day<'_> for Day22 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            secrets: input.lines().filter_map(|l| l.parse().ok()).collect(),
            all_prices: Cell::new(vec![]),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let all_secrets = self
            .secrets
            .iter()
            .map(|&secret| {
                (0..2000)
                    .scan(secret, |v, _| {
                        *v = compute(*v);
                        Some(*v)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        self.all_prices.set(
            all_secrets
                .iter()
                .map(|secrets| secrets.iter().map(|&secret| (secret % 10) as i8).collect())
                .collect(),
        );
        all_secrets.iter().map(|secrets| secrets.last().unwrap().clone()).sum()
    }

    fn part_2(&self) -> Self::T2 {
        // prices diffs sequences
        // [0]5
        // [1]3   [0]-2
        // [2]7   [1] 4
        // [3]1   [2]-3
        // [4]2   [3] 1 [0]-2,4,-3,1
        // [5]4   [4] 3 [1]4,-3,1,4
        // [6]2   [5]-2 [2]-3,1,3,-2
        // sequences[i] -> prices[i + 4]

        self.all_prices
            .take()
            .iter()
            .enumerate()
            .flat_map(|(n, prices)| {
                prices
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<_>>()
                    .windows(4)
                    .map(|w| {
                        w.iter()
                            .enumerate()
                            .map(|(i, &e)| 20usize.pow(i as u32) * (10 + e) as usize)
                            .sum::<usize>()
                    })
                    .enumerate()
                    .map(|(i, sequence)| (n, sequence, prices[i + 4] as usize))
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .fold(AHashMap::new(), |mut prices_by_sequence, (n, sequence, price)| {
                prices_by_sequence
                    .entry(sequence)
                    .and_modify(|prices: &mut AHashMap<usize, usize>| {
                        prices.entry(n).or_insert(price);
                    })
                    .or_insert_with(|| AHashMap::from([(n, price)]));
                prices_by_sequence
            })
            .values()
            .map(|prices_by_buyer| prices_by_buyer.values().sum())
            .max()
            .unwrap()
    }
}
