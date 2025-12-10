use crate::day::Day;
use crate::util::SortableByKey;

pub struct Day05 {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

impl Day<'_> for Day05 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (ranges, ids) = input.split_once("\n\n").unwrap();
        Self {
            ranges: ranges
                .lines()
                .filter_map(|l| l.split_once('-'))
                .filter_map(|(f, t)| Some((f.parse().ok()?, t.parse().ok()?)))
                .collect(),
            ids: ids.lines().filter_map(|l| l.parse().ok()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.ids
            .iter()
            .filter(|&id| self.ranges.iter().any(|&(f, t)| (f..=t).contains(id)))
            .count()
    }

    fn part_2(&self) -> Self::T2 {
        self.ranges
            .iter()
            .sorted_unstable_by_key(|&(f, _)| f)
            .fold(Vec::<(usize, usize)>::new(), |mut merged, &(f, t)| {
                if merged.is_empty() {
                    merged.push((f, t));
                } else {
                    let last_index = merged.len() - 1;
                    let previous_t = &mut merged[last_index].1;
                    if *previous_t >= f {
                        *previous_t = (*previous_t).max(t);
                    } else {
                        merged.push((f, t));
                    }
                }
                merged
            })
            .into_iter()
            .map(|(f, t)| t - f + 1)
            .sum()
    }
}
