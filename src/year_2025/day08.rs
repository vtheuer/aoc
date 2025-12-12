use crate::day::Day;
use crate::util::{Joinable, SortableByKey};
use ahash::{AHashMap, AHashSet};
use std::collections::BTreeSet;
use std::convert::identity;

pub struct Day08 {
    boxes: Vec<(usize, usize, usize)>,
}

fn root(parents: &[usize], i: usize) -> usize {
    let mut r = i;
    while parents[r] != r {
        r = parents[r];
    }
    r
}

impl Day<'_> for Day08 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            boxes: input
                .lines()
                .filter_map(|l| {
                    let mut ns = l.split(',').filter_map(|n| n.parse().ok());
                    Some((ns.next()?, ns.next()?, ns.next()?))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let box_count = self.boxes.len();
        let mut distances = Vec::new();

        for i in 0..box_count {
            for j in i + 1..box_count {
                let a = self.boxes[i];
                let b = self.boxes[j];
                let distance = a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2);
                distances.push((distance, i, j));
            }
        }
        distances.sort_by_key(|&(d, _, _)| d);
        let mut parents = (0..box_count).collect::<Vec<_>>();

        for &(_, i, j) in distances.iter().take(10) {
            parents[j] = root(&parents, i);
            println!("({}, {}) -> parents[{}] = {}", i, j, j, parents[j]);
        }

        println!("{}", parents.iter().enumerate().map(|(i, p)| format!("{i} -> {p}")).join("\n"));
        let counts: AHashMap<usize, usize> = parents.into_iter().fold(AHashMap::new(), |mut counts, root| {
            counts.entry(root).and_modify(|c| *c += 1).or_insert(1);
            counts
        });

        counts.values().sorted_unstable_by_key(|&&c| c).rev().take(3).map(|c| dbg!(c)).product()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
