use crate::day::Day;
use crate::util::{Joinable, SortableByKey};
use ahash::AHashMap;

pub struct Day08 {
    boxes: Vec<(usize, usize, usize)>,
    distances: Vec<(usize, usize, usize)>,
}

trait UnionFind {
    fn parent(&self, x: usize) -> usize;

    fn parent_mut(&mut self, x: usize) -> &mut usize;
    fn find(&mut self, x: usize) -> usize;
    fn union(&mut self, x: usize, y: usize);
}

impl UnionFind for Vec<usize> {
    fn parent(&self, x: usize) -> usize {
        self[x]
    }

    fn parent_mut(&mut self, x: usize) -> &mut usize {
        &mut self[x]
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent(x) != x {
            *self.parent_mut(x) = self.find(self.parent(x));
        }
        self.parent(x)
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        if x_root < y_root {
            *self.parent_mut(y_root) = x_root;
        } else {
            *self.parent_mut(x_root) = y_root;
        }
    }
}

impl Day<'_> for Day08 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let boxes = input
            .lines()
            .filter_map(|l| {
                let mut ns = l.split(',').filter_map(|n| n.parse::<usize>().ok());
                Some((ns.next()?, ns.next()?, ns.next()?))
            })
            .collect::<Vec<_>>();
        let box_count = boxes.len();

        Self {
            distances: (0..box_count)
                .flat_map(|i| (i + 1..box_count).map(move |j| (i, j)))
                .map(|(i, j)| {
                    let a = boxes[i];
                    let b = boxes[j];
                    (
                        a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2),
                        i,
                        j,
                    )
                })
                .sorted_unstable_by_key(|(d, _, _)| *d)
                .collect(),
            boxes,
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut parents = (0..self.boxes.len()).collect::<Vec<_>>();

        for &(_, i, j) in self.distances.iter().take(1000) {
            parents.union(i, j);
        }

        let counts = (0..parents.len()).fold(AHashMap::new(), |mut counts, i| {
            counts.entry(parents.find(i)).and_modify(|c| *c += 1).or_insert(1);
            counts
        });

        counts.values().sorted_unstable_by_key(|&&c| c).rev().take(3).product()
    }

    fn part_2(&self) -> Self::T2 {
        let mut parents = (0..self.boxes.len()).collect::<Vec<_>>();

        for &(_, i, j) in self.distances.iter() {
            parents.union(i, j);
            if (0..parents.len())
                .filter(|&p| parents.find(p) == p)
                .take(2)
                .count() == 1 {
                return self.boxes[i].0 * self.boxes[j].0
            }
        }

        unreachable!()
    }
}
