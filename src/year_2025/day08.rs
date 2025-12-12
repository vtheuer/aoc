use crate::day::Day;
use crate::util::{Joinable, SortableByKey};
use ahash::{AHashMap, AHashSet};
use std::collections::BTreeSet;
use std::convert::identity;

pub struct Day08 {
    boxes: Vec<(usize, usize, usize)>,
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
                distances.push(if a < b { (distance, i, j) } else { (distance, j, i) });
            }
        }
        distances.sort_by_key(|&(d, _, _)| d);
        let mut parents = (0..box_count).collect::<Vec<_>>();
        let mut friends = vec![1; box_count];

        for &(distance, ai, bi) in distances.iter().take(10) {
            // println!("({},{},{}) -> ({},{},{}) = {} :", a.0, a.1, a.2, b.0, b.1, b.2, distance);
            // println!("{}\n", groups.iter().map(|g| g.iter().map(|b| format!("({},{},{})", b.0, b.1, b.2)).join(", ")).join("\n"));
            parents[ai] = bi;
        }
        println!("{}", parents.into_iter().enumerate().map(|(i, p)| format!("{i} -> {p}")).join("\n"));
        0
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
