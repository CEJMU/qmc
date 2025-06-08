#[derive(Debug, Clone, PartialEq)]
pub enum Bit {
    Zero,
    One,
    DontCare,
}

pub fn hamming(a: &Vec<Bit>, b: &Vec<Bit>) -> Option<Vec<Bit>> {
    a.iter()
        .zip(b)
        .map(|(bit_a, bit_b)| match (bit_a, bit_b) {
            (Bit::Zero, Bit::One) | (Bit::One, Bit::Zero) => Some(Bit::DontCare),
            (Bit::Zero, Bit::Zero) => Some(Bit::Zero),
            (Bit::One, Bit::One) => Some(Bit::One),
            (Bit::DontCare, Bit::DontCare) => Some(Bit::DontCare),
            _ => None,
        })
        .collect()
}

pub fn compare_sets(set_a: &Vec<Vec<Bit>>, set_b: &Vec<Vec<Bit>>) -> Option<Vec<Vec<Bit>>> {
    // The sets set_a and set_b can have different lenght. Now we compare each element of set_a with each element of set_b, by calling the hamming function. If there is a match, we store the result in a new set. If it returns None, then we go to the next element of set_b or the next element of set_a.
    let mut result_set = Vec::new();

    for a in set_a {
        for b in set_b {
            if let Some(result) = hamming(a, b) {
                result_set.push(result);
            }
        }
    }
    Some(result_set)
}
