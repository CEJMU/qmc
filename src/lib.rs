use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Bit {
    Zero,
    One,
    DontCare,
}

#[derive(PartialEq, Debug)]
pub struct BitSet {
    pub bits: Vec<Bit>,
    pub used: bool,
}

pub fn hamming(a: &BitSet, b: &BitSet) -> Option<BitSet> {
    let bits = a
        .bits
        .iter()
        .zip(b.bits.iter())
        .map(|(bit_a, bit_b)| match (bit_a, bit_b) {
            (Bit::Zero, Bit::One) | (Bit::One, Bit::Zero) => Some(Bit::DontCare),
            (Bit::Zero, Bit::Zero) => Some(Bit::Zero),
            (Bit::One, Bit::One) => Some(Bit::One),
            (Bit::DontCare, Bit::DontCare) => Some(Bit::DontCare),
            _ => None,
        })
        .collect::<Option<Vec<Bit>>>()?;

    Some(BitSet { bits, used: false })
}

pub fn compare_sets(set_a: &[BitSet], set_b: &[BitSet]) -> Option<Vec<BitSet>> {
    let result_set: Vec<_> = set_a
        .iter()
        .flat_map(|a| {
            set_b.iter().filter_map(move |b| hamming(a, b)) // filters None out
        })
        .collect();

    Some(result_set)
}

pub fn parse(bits: &[Vec<i32>]) -> Vec<Vec<Bit>> {
    bits.iter()
        .map(|bit_vec| {
            bit_vec
                .iter()
                .map(|&b| if b == 1 { Bit::One } else { Bit::Zero })
                .collect()
        })
        .collect()
}

pub fn sort_bits(bits: &[Vec<Bit>]) -> Vec<Vec<BitSet>> {
    let mut groups: HashMap<usize, Vec<Vec<Bit>>> = HashMap::new();

    for vec in bits {
        let count = vec.iter().filter(|bit| **bit == Bit::One).count();
        groups.entry(count).or_default().push(vec.clone());
    }

    // Now collect in sorted order
    let mut result = Vec::new();
    let mut keys: Vec<_> = groups.keys().copied().collect();
    keys.sort(); // sort by the number of ones

    for key in keys {
        if let Some(group) = groups.remove(&key) {
            let bitsets = group
                .into_iter()
                .map(|bits| BitSet { bits, used: false })
                .collect();
            result.push(bitsets);
        }
    }

    result
}

pub fn find_prime_implicants(groups: Vec<Vec<BitSet>>) -> Vec<BitSet> {
    let mut result = Vec::new();

    for window in groups.windows(2) {
        let a = &window[0];
        let b = &window[1];

        if let Some(comparison_result) = compare_sets(a, b) {
            result.extend(comparison_result);
        }
    }

    result
}
