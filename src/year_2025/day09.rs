use crate::day::Day;

pub struct Day09 {
    corners: Vec<(usize, usize)>,
}

impl Day<'_> for Day09 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            corners: input
                .lines()
                .filter_map(|l| {
                    let (x, y) = l.split_once(',')?;
                    Some((x.parse().ok()?, y.parse().ok()?))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.corners
            .iter()
            .enumerate()
            .flat_map(|(i, a)| self.corners[i + 1..].iter().map(move |b| (a, b)))
            .map(|(&(ax, ay), &(bx, by))| (ax.abs_diff(bx) + 1) * (ay.abs_diff(by) + 1))
            .max()
            .unwrap()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
