use crate::day::Day;
use crate::util::{first_line, FindIndex};

pub struct Day07 {
    splitters: Vec<Vec<usize>>,
    start: usize,
    width: usize,
}

impl Day<'_> for Day07 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            splitters: input
                .lines()
                .map(|l| l.bytes().enumerate().filter(|&(_, c)| c == b'^').map(|(i, _)| i).collect::<Vec<_>>())
                .filter(|splitters| !splitters.is_empty())
                .collect(),
            start: first_line(input).bytes().find_index(b'S').unwrap(),
            width: first_line(input).len()
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut initial_beams = vec![false; self.width];
        initial_beams[self.start] = true;
        self.splitters
            .iter()
            .fold((initial_beams, 0), |(mut beams, mut splits), splitters| {
                for &s in splitters {
                    let b = beams[s];
                    if b {
                        splits += 1;
                        beams[s] = false;
                        beams[s - 1] = true;
                        beams[s + 1] = true;
                    }
                }
                (beams, splits)
            })
            .1
    }

    fn part_2(&self) -> Self::T2 {
        let mut initial_beams = vec![0; self.width];
        initial_beams[self.start] = 1;
        self.splitters
            .iter()
            .fold(initial_beams, |mut beams, splitters| {
                for &s in splitters {
                    let n = beams[s];
                    if n > 0 {
                        beams[s] = 0;
                        beams[s - 1] += n;
                        beams[s + 1] += n;
                    }
                }
                beams
            })
            .into_iter()
            .sum()
    }
}
