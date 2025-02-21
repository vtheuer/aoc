use crate::day::Day;
use crate::util::Joinable;
use ahash::{AHashMap, AHashSet};

pub struct Day23<'a> {
    pcs: Vec<&'a str>,
    matrix: Vec<Vec<bool>>,
}

impl<'a> Day<'a> for Day23<'a> {
    type T1 = usize;
    type T2 = String;

    fn new(input: &'a str) -> Self {
        let mut connections = input
            .lines()
            .filter_map(|l| l.split_once('-'))
            .map(|(a, b)| if a <= b { (a, b) } else { (b, a) })
            .collect::<Vec<_>>();
        connections.sort();

        let mut pcs = connections
            .iter()
            .flat_map(|&(a, b)| [a, b].into_iter())
            .collect::<AHashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        pcs.sort();

        let mut matrix = vec![vec![false; pcs.len()]; pcs.len()];
        for (i, a) in pcs.iter().enumerate() {
            for (j, b) in pcs.iter().enumerate().skip(i) {
                if connections.binary_search(&(a, b)).is_ok() {
                    matrix[i][j] = true;
                }
            }
        }
        Self { pcs, matrix }
    }

    fn part_1(&self) -> Self::T1 {
        self.matrix
            .iter()
            .enumerate()
            .map(|(i, neighbors)| {
                neighbors
                    .iter()
                    .enumerate()
                    .skip(i)
                    .filter(|&(_, &c)| c)
                    .map(|(j, _)| {
                        neighbors
                            .iter()
                            .enumerate()
                            .skip(j)
                            .filter(|&(_, &c)| c)
                            .filter(|&(k, _)| self.matrix[j][k])
                            .filter(|&(k, _)| {
                                self.pcs[i].starts_with('t')
                                    || self.pcs[j].starts_with('t')
                                    || self.pcs[k].starts_with('t')
                            })
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        let mut groups = vec![];
        for (i, connections) in self.matrix.iter().enumerate() {
            for (j, _) in connections.iter().enumerate().skip(i).filter(|&(_, &c)| c) {
                let mut group = vec![i, j];
                for (k, _) in connections.iter().enumerate().skip(j).filter(|&(_, &c)| c) {
                    if group.iter().all(|&o| self.matrix[o][k]) {
                        group.push(k);
                    }
                }
                if group.len() > 2 {
                    groups.push(group);
                }
            }
        }

        groups
            .into_iter()
            .max_by_key(|g| g.len())
            .map(|g| g.into_iter().map(|i| self.pcs[i]).join(","))
            .unwrap()
    }
}
