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

#[cfg(test)]
mod tests {
    use crate::util::Joinable;
    use std::io::BufRead;

    #[test]
    fn test() {
        let input = "ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
            .lines()
            .map(|l| l.split(' ').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        println!("digraph {{");

        println!(
            "{}",
            input
                .iter()
                .enumerate()
                .map(|(i, l)| format!("  {i} [label=\"{}\"]", l[1]))
                .join("\n")
        );

        // let from = input
        //     .iter()
        //     .enumerate()
        //     .map(|(i, l)| (l[4], i))
        //     .collect::<AHashMap<_, _>>();
        //
        // let to = input
        //     .iter()
        //     .enumerate()
        //     .flat_map(|(i, l)| [l[0], l[2]].into_iter().map(move |w| (w, i)))
        //     .collect::<AHashMap<_, _>>();

        println!(
            "{}",
            input
                .iter()
                .enumerate()
                .flat_map(|(i, a)| {
                    let out = a[4];
                    input
                        .iter()
                        .enumerate()
                        .filter(move |(_, b)| b[0] == out || b[2] == out)
                        .map(move |(j, _)| format!("  {i} -> {j} [label=\"{out}\"]"))
                })
                .join("\n")
        );

        println!(
            "{}",
            input
                .iter()
                .enumerate()
                .flat_map(|(i, l)| [l[0], l[2]].into_iter().map(move |w| (i, w)))
                .filter(|(_, w)| w.starts_with('x') || w.starts_with('y'))
                .map(|(i, w)| format!("  {} -> {i}", w))
                .join("\n")
        );

        println!(
            "{}",
            input
                .iter()
                .enumerate()
                .filter(|(_, l)| l[4].starts_with('z'))
                .map(|(i, l)| format!("  {i} -> {}", l[4]))
                .join("\n")
        );

        println!("}}");
    }
}
