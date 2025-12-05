use crate::day::Day;

#[derive(Debug)]
pub struct Day01 {
    rotations: Vec<(bool, isize)>,
}

impl Day<'_> for Day01 {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &str) -> Self {
        Self {
            rotations: input
                .lines()
                .map(|l| {
                    let (dir, n) = l.split_at(1);
                    (dir == "R", n.parse::<isize>().unwrap())
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.rotations
            .iter()
            .fold((50, 0), |(position, zeros), &(clockwise, count)| {
                let mut new_position = (position + if clockwise { count } else { -count }) % 100;
                new_position = if new_position < 0 {
                    100 + new_position
                } else {
                    new_position
                };
                (new_position, zeros + if new_position == 0 { 1 } else { 0 })
            })
            .1
    }

    fn part_2(&self) -> Self::T2 {
        self.rotations
            .iter()
            .fold((50isize, 0), |(position, mut zeros), &(clockwise, count)| {
                let mut new_position = position + if clockwise { count } else { -count };
                if clockwise {
                    zeros += new_position / 100;
                } else {
                    zeros += (new_position / 100).abs();
                }
                zeros += if position != 0 && position.signum() != new_position.signum() {
                    1
                } else {
                    0
                };
                new_position %= 100;
                new_position = if new_position < 0 {
                    100 + new_position
                } else {
                    new_position
                };
                (new_position, zeros)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
