use ahash::{AHashMap, AHashSet};

use crate::day::Day;
use crate::util::{Joinable, SortableByKey};

pub struct Day21<'a> {
    recipes: Vec<(AHashSet<&'a str>, AHashSet<&'a str>)>,
    allergenic_ingredients: AHashMap<&'a str, &'a str>,
}

impl<'a> Day<'a> for Day21<'a> {
    type T1 = u16;
    type T2 = String;

    fn new(input: &'a str) -> Self {
        let recipes = input
            .lines()
            .map(|l| {
                let (ingredients, allergens) = l.split_once(" (contains ")?;
                Some((
                    ingredients.split(' ').collect(),
                    allergens[..allergens.len() - 1].split(", ").collect(),
                ))
            })
            .map(Option::unwrap)
            .collect::<Vec<(AHashSet<_>, AHashSet<_>)>>();
        Day21 {
            allergenic_ingredients: recipes
                .iter()
                .flat_map(|(_, allergens)| allergens.iter())
                .collect::<AHashSet<_>>()
                .into_iter()
                .map(|&allergen| {
                    (
                        allergen,
                        recipes
                            .iter()
                            .filter(|(_, a)| a.contains(&allergen))
                            .map(|(i, _)| i)
                            .fold(AHashSet::default(), |common, ingredients| {
                                if common.is_empty() {
                                    ingredients.clone()
                                } else {
                                    common.intersection(ingredients).copied().collect()
                                }
                            }),
                    )
                })
                .sorted_unstable_by_key(|(_, possible_ingredients)| possible_ingredients.len())
                .fold(AHashMap::default(), |mut r, (allergen, possible_ingredients)| {
                    r.insert(
                        possible_ingredients.into_iter().find(|i| !r.contains_key(i)).unwrap(),
                        allergen,
                    );
                    r
                }),
            recipes,
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.recipes
            .iter()
            .flat_map(|(ingredients, _)| ingredients.iter())
            .filter(|&&ingredient| !self.allergenic_ingredients.contains_key(ingredient))
            .fold(AHashMap::default(), |mut counts, &ingredient| {
                *counts.entry(ingredient).or_insert(0) += 1;
                counts
            })
            .into_iter()
            .map(|(_, count)| count)
            .sum::<Self::T1>()
    }

    fn part_2(&self) -> Self::T2 {
        self.allergenic_ingredients
            .iter()
            .sorted_unstable_by_key(|&(_, &allergen)| allergen)
            .map(|(&ingredient, _)| ingredient)
            .join(",")
    }
}
