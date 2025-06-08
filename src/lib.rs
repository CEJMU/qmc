use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Bit {
    Zero,
    One,
    DontCare,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct BitSet {
    pub bits: Vec<Bit>,
    pub used: bool,
}

pub fn hamming(a: &BitSet, b: &BitSet) -> Option<BitSet> {
    let mut bits = Vec::new();
    let mut diff_count = 0;

    for (bit_a, bit_b) in a.bits.iter().zip(b.bits.iter()) {
        match (bit_a, bit_b) {
            (Bit::Zero, Bit::One) | (Bit::One, Bit::Zero) => {
                diff_count += 1;
                bits.push(Bit::DontCare);
            }
            (Bit::Zero, Bit::Zero) => bits.push(Bit::Zero),
            (Bit::One, Bit::One) => bits.push(Bit::One),
            (Bit::DontCare, Bit::DontCare) => bits.push(Bit::DontCare),
            _ => return None, // z. B. (Zero, DontCare)
        }
    }

    if diff_count == 1 {
        Some(BitSet { bits, used: false })
    } else {
        None
    }
}

pub fn compare_sets(set_a: &mut [BitSet], set_b: &mut [BitSet]) -> Option<Vec<BitSet>> {
    let mut result = Vec::new();

    for a in set_a.iter_mut() {
        for b in set_b.iter_mut() {
            if let Some(new) = hamming(a, b) {
                a.used = true;
                b.used = true;
                result.push(new);
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

pub fn parse(bits: &[Vec<i32>]) -> Vec<BitSet> {
    bits.iter()
        .map(|bit_vec| {
            let bits = bit_vec
                .iter()
                .map(|&b| if b == 1 { Bit::One } else { Bit::Zero })
                .collect();
            BitSet { bits, used: false }
        })
        .collect()
}

pub fn sort_bits(bitsets: &[BitSet]) -> Vec<Vec<BitSet>> {
    let mut groups: HashMap<usize, Vec<BitSet>> = HashMap::new();

    for bitset in bitsets {
        let count = bitset.bits.iter().filter(|bit| **bit == Bit::One).count();
        groups.entry(count).or_default().push(bitset.clone()); // wichtig: clone(), um Ownership zu erhalten
    }

    // Jetzt in sortierter Reihenfolge einsammeln
    let mut result = Vec::new();
    let mut keys: Vec<_> = groups.keys().copied().collect();
    keys.sort();

    for key in keys {
        if let Some(group) = groups.remove(&key) {
            result.push(group);
        }
    }

    result
}

pub fn find_prime_implicants(groups: &mut [Vec<BitSet>]) -> Vec<BitSet> {
    let mut result = Vec::new();

    for i in 0..groups.len().saturating_sub(1) {
        let (left, right) = groups.split_at_mut(i + 1);
        let a = &mut left[i];
        let b = &mut right[0];

        if let Some(comparison_result) = compare_sets(a, b) {
            result.extend(comparison_result);
        }
    }

    // Duplikate entfernen:
    let unique: HashSet<BitSet> = result.into_iter().collect();
    unique.into_iter().collect()
}

pub fn visualize_bits(bitsets: &Vec<BitSet>) {
    for set in bitsets {
        let mut result = String::new();
        for bit in &set.bits {
            result.push(match bit {
                Bit::Zero => '0',
                Bit::One => '1',
                Bit::DontCare => '-',
            });
        }
        if set.used {
            result.push_str(" ✅");
        } else {
            result.push_str(" 🥇");
        }
        println!("{}", result);
    }
}

pub fn search_prime_implicants(bits: Vec<BitSet>) -> Vec<BitSet> {
    let mut grouped = sort_bits(&bits);
    let mut primes = Vec::new();

    loop {
        let mut groups = grouped;
        let new_combinations = find_prime_implicants(&mut groups);

        // Alle unbenutzten ursprünglichen BitSets sind Prime Implicants
        for group in &groups {
            for bitset in group {
                if !bitset.used {
                    primes.push(bitset.clone());
                }
            }
        }

        if new_combinations.is_empty() {
            break;
        }

        grouped = sort_bits(&new_combinations);
    }

    // Duplikate entfernen
    let unique: HashSet<_> = primes.into_iter().collect();
    unique.into_iter().collect()
}
