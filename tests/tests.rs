use qmc::{Bit, compare_sets, hamming};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming() {
        // Test the generation of a Dontcare:
        let a = vec![Bit::Zero, Bit::One, Bit::Zero, Bit::DontCare];
        let b = vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::DontCare];
        let e: Vec<Bit> = vec![Bit::Zero, Bit::DontCare, Bit::Zero, Bit::DontCare];
        assert_eq!(hamming(&a, &b), Some(e));

        // Test the case where DontCares don't lign up:
        let a = vec![Bit::Zero, Bit::One, Bit::Zero];
        let b = vec![Bit::DontCare, Bit::One, Bit::Zero];
        assert_eq!(hamming(&a, &b), None);
    }

    #[test]
    fn test_compare_sets() {
        let set_a = vec![vec![Bit::Zero, Bit::Zero]];
        let set_b = vec![vec![Bit::Zero, Bit::One], vec![Bit::One, Bit::Zero]];
        let set_e = vec![
            vec![Bit::Zero, Bit::DontCare],
            vec![Bit::DontCare, Bit::Zero],
        ];
        assert_eq!(compare_sets(&set_a, &set_b), Some(set_e));
    }
}
