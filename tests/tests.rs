use qmc::{Bit, BitSet, compare_sets, hamming, parse, sort_bits};

#[cfg(test)]
mod tests {
    use qmc::find_prime_implicants;

    use super::*;

    #[test]
    fn test_hamming_generate() {
        // Test the generation of a Dontcare:
        let a = BitSet {
            bits: vec![Bit::Zero, Bit::One, Bit::Zero, Bit::DontCare],
            used: false,
        };
        let b = BitSet {
            bits: vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::DontCare],
            used: false,
        };
        let e = BitSet {
            bits: vec![Bit::Zero, Bit::DontCare, Bit::Zero, Bit::DontCare],
            used: false,
        };
        assert_eq!(hamming(&a, &b), Some(e));
    }

    #[test]
    fn test_hamming_ignore() {
        // Test the case where DontCares don't lign up:
        let a = BitSet {
            bits: vec![Bit::Zero, Bit::One, Bit::Zero],
            used: false,
        };
        let b = BitSet {
            bits: vec![Bit::DontCare, Bit::One, Bit::Zero],
            used: false,
        };
        assert_eq!(hamming(&a, &b), None);
    }

    #[test]
    fn test_compare_sets() {
        let mut set_a = vec![BitSet {
            bits: vec![Bit::Zero, Bit::Zero],
            used: false,
        }];
        let mut set_b = vec![
            BitSet {
                bits: vec![Bit::Zero, Bit::One],
                used: false,
            },
            BitSet {
                bits: vec![Bit::One, Bit::Zero],
                used: false,
            },
        ];
        let set_e = vec![
            BitSet {
                bits: vec![Bit::Zero, Bit::DontCare],
                used: false,
            },
            BitSet {
                bits: vec![Bit::DontCare, Bit::Zero],
                used: false,
            },
        ];
        assert_eq!(compare_sets(&mut set_a, &mut set_b), Some(set_e));
    }

    #[test]
    fn test_parse() {
        let bits = vec![
            vec![0, 0, 0], //
            vec![0, 0, 1], //
            vec![1, 0, 0], //
        ];

        let expected = vec![
            BitSet {
                bits: vec![Bit::Zero, Bit::Zero, Bit::Zero],
                used: false,
            },
            BitSet {
                bits: vec![Bit::Zero, Bit::Zero, Bit::One],
                used: false,
            },
            BitSet {
                bits: vec![Bit::One, Bit::Zero, Bit::Zero],
                used: false,
            },
        ];

        assert_eq!(parse(&bits), expected);
    }

    #[test]
    fn test_sort() {
        let bits = vec![
            BitSet {
                bits: vec![Bit::Zero, Bit::Zero, Bit::One],
                used: false,
            }, //
            BitSet {
                bits: vec![Bit::Zero, Bit::Zero, Bit::Zero],
                used: false,
            }, //
            BitSet {
                bits: vec![Bit::One, Bit::Zero, Bit::Zero],
                used: false,
            }, //
            BitSet {
                bits: vec![Bit::One, Bit::Zero, Bit::One],
                used: false,
            }, //
        ];

        let expected = vec![
            vec![BitSet {
                bits: vec![Bit::Zero, Bit::Zero, Bit::Zero],
                used: false,
            }],
            vec![
                BitSet {
                    bits: vec![Bit::Zero, Bit::Zero, Bit::One],
                    used: false,
                },
                BitSet {
                    bits: vec![Bit::One, Bit::Zero, Bit::Zero],
                    used: false,
                },
            ],
            vec![BitSet {
                bits: vec![Bit::One, Bit::Zero, Bit::One],
                used: false,
            }],
        ];

        assert_eq!(sort_bits(&bits), expected);
    }

    #[test]
    fn test_find_prime_implicants() {
        let mut bits = vec![
            vec![BitSet {
                bits: vec![Bit::Zero, Bit::Zero],
                used: false,
            }],
            vec![BitSet {
                bits: vec![Bit::Zero, Bit::One],
                used: false,
            }],
        ];

        let expected = vec![BitSet {
            bits: vec![Bit::Zero, Bit::DontCare],
            used: false,
        }];

        assert_eq!(find_prime_implicants(&mut bits), expected);
    }
}
