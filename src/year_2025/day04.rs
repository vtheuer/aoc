use crate::day::Day;
use crate::util::grid::Grid;

pub struct Day04 {
    grid: Grid<bool>,
}

impl Day<'_> for Day04 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            grid: Grid::parse(input, |c| c == b'@'),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.grid
            .indices()
            .filter(|&i| self.grid[i] && self.grid.neighboring_indices(i, true).filter(|&j| self.grid[j]).count() < 4)
            .count()
    }

    fn part_2(&self) -> Self::T2 {
        let mut grid = self.grid.clone();
        let mut total_removed = 0;

        loop {
            let (removed, new_grid) = grid.indices().fold(
                (0, Grid::init(grid.width, grid.height, false)),
                |(r, mut new_grid), i| {
                    let before = grid[i];
                    let after = before && grid.neighboring_indices(i, true).filter(|&j| grid[j]).count() >= 4;
                    new_grid[i] = after;
                    (r + if before && !after { 1 } else { 0 }, new_grid)
                },
            );
            total_removed += removed;
            grid = new_grid;
            if removed == 0 {
                break;
            }
        }

        total_removed
    }
}
