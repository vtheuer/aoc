use crate::day::Day;
use crate::util::grid::Grid;

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
        let (max_x, max_y) = self.corners.iter()
            .fold((0, 0), |(max_x, max_y), &(x, y)| {
                (max_x.max(x), max_y.max(y))
            });
        dbg!(self.corners.iter()
            .fold((usize::MAX, usize::MAX), |(min_x, min_y), &(x, y)| {
                (min_x.min(x), min_y.min(y))
            }));
        let mut grid = Grid::init(max_x + 1, max_y + 1, false);

        for i in 0..self.corners.len() {
            let (ax, ay) = self.corners[i];
            let (bx, by) = self.corners[(i + 1) % self.corners.len()];
            println!("{},{} to {},{}", ax, ay, bx, by);
            if ax != bx {
                for x in ax.min(bx)..ax.max(bx) {
                    grid[(x, ay)] = true;
                }
            } else {
                for y in ay.min(by)..ay.max(by) {
                    grid[(ax, y)] = true;
                }
            }
        }

        println!("{}", grid.format(|_, &b| if b {'#'} else {'.'}));

        0
    }
}
