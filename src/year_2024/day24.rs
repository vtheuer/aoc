use crate::day::Day;
use ahash::AHashMap;

#[derive(Debug, Copy, Clone)]
enum GateType {
    And,
    Or,
    Xor,
}
use GateType::*;

impl GateType {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            And => a && b,
            Or => a || b,
            Xor => a ^ b,
        }
    }
}

pub struct Day24<'a> {
    wires: AHashMap<&'a str, bool>,
    gates: AHashMap<&'a str, (GateType, &'a str, &'a str)>,
}

impl<'a> Day24<'a> {
    fn get(&self, wire: &'a str, cache: &mut AHashMap<&'a str, bool>) -> bool {
        if let Some(&r) = cache.get(wire) {
            r
        } else if let Some((gate, a, b)) = self.gates.get(wire) {
            let r = gate.apply(self.get(a, cache), self.get(b, cache));
            cache.insert(wire, r);
            r
        } else {
            self.wires[wire]
        }
    }
}

impl<'a> Day<'a> for Day24<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let (wires, gates) = input.split_once("\n\n").unwrap();
        Self {
            wires: wires
                .lines()
                .filter_map(|l| l.split_once(": "))
                .map(|(name, value)| (name, value == "1"))
                .collect(),
            gates: gates
                .lines()
                .filter_map(|l| {
                    let (gate, out) = l.split_once(" -> ")?;
                    let mut parts = gate.split(' ');
                    let in_1 = parts.next()?;
                    Some((
                        out,
                        (
                            match parts.next()? {
                                "AND" => And,
                                "OR" => Or,
                                "XOR" => Xor,
                                _ => unreachable!(),
                            },
                            in_1,
                            parts.next()?,
                        ),
                    ))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut zs = AHashMap::new();
        let mut cache = AHashMap::new();
        for &z in self.gates.keys().filter(|o| o.starts_with('z')) {
            zs.insert(z, self.get(z, &mut cache));
        }

        zs.into_iter().fold(0, |n, (wire, b)| {
            n | ((if b { 1 } else { 0 }) << wire[1..].parse::<u32>().unwrap())
        })
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
