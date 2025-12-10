use crate::day::Day;

pub struct Day06<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Day<'a> for Day06<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let rows = self
            .lines
            .iter()
            .map(|&l| l.split_ascii_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        (0..rows[0].len())
            .map(|i| rows.iter().map(|row| row[i]).collect::<Vec<_>>())
            .map(|c| {
                let operands = c[0..c.len() - 1].iter().filter_map(|n| n.parse::<usize>().ok());
                if c.last().unwrap().bytes().next().unwrap() == b'+' {
                    operands.sum::<usize>()
                } else {
                    operands.product()
                }
            })
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        let operator_line = self.lines.last().unwrap().as_bytes();
        let operators_with_starts = operator_line
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c != b' ')
            .map(|(i, &c)| (c, i))
            .collect::<Vec<_>>();
        let operand_lines = &self.lines[0..self.lines.len() - 1];

        (0..operators_with_starts.len())
            .map(|i| {
                let (o, f) = operators_with_starts[i];
                (
                    o,
                    f,
                    if i < operators_with_starts.len() - 1 {
                        operators_with_starts[i + 1].1 - 2
                    } else {
                        operator_line.len()
                    },
                )
            })
            .map(|(o, f, t)| {
                let operands = (f..=t)
                    .rev()
                    .map(|i| {
                        operand_lines
                            .iter()
                            .map(|&l| if i < l.len() { l.as_bytes()[i] } else { b' ' })
                            .filter(|&c| c != b' ')
                            .fold(0usize, |n, d| n * 10 + (d - b'0') as usize)
                    })
                    .filter(|&n| n > 0);
                if o == b'+' {
                    operands.sum::<usize>()
                } else {
                    operands.product()
                }
            })
            .sum()
    }
}
