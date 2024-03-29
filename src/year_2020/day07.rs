use ahash::{AHashMap, AHashSet};

use crate::day::Day;

pub struct Day07<'a> {
    rules: AHashMap<&'a str, AHashMap<&'a str, u32>>,
}

impl Day07<'_> {
    fn count_bags_in(&self, container: &str) -> u32 {
        self.rules
            .get(container)
            .unwrap()
            .iter()
            .map(|(bag, count)| *count * (1 + self.count_bags_in(bag)))
            .sum()
    }
}

fn bags_containing<'a>(containers_by_bag: &AHashMap<&str, Vec<&'a str>>, bag: &str) -> AHashSet<&'a str> {
    containers_by_bag
        .get(bag)
        .map(|bags| {
            bags.iter().fold(AHashSet::default(), |mut containers, container| {
                containers.insert(*container);
                containers.extend(bags_containing(containers_by_bag, container));
                containers
            })
        })
        .unwrap_or_else(AHashSet::default)
}

impl<'a> Day<'a> for Day07<'a> {
    type T1 = usize;
    type T2 = u32;

    fn new(input: &'a str) -> Self {
        Day07::<'a> {
            rules: input
                .lines()
                .map(|l| {
                    let (bag, content) = l.split_once(" bags contain ")?;
                    Some((
                        bag,
                        content
                            .split(", ")
                            .filter_map(|content| {
                                let input = content;
                                let (count_and_bag, _) = input.rsplit_once(' ')?;
                                let (count, bag) = count_and_bag.split_once(' ')?;

                                Some((bag, count.parse().ok()?))
                            })
                            .collect(),
                    ))
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        bags_containing(
            &self
                .rules
                .iter()
                .flat_map(|(container, content)| content.iter().map(move |(bag, _)| (*bag, *container)))
                .fold(AHashMap::default(), |mut containers_by_bag, (bag, container)| {
                    containers_by_bag.entry(bag).or_insert_with(Vec::new).push(container);
                    containers_by_bag
                }),
            "shiny gold",
        )
        .len()
    }

    fn part_2(&self) -> Self::T2 {
        self.count_bags_in("shiny gold")
    }
}
