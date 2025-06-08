use qmc::{parse, search_prime_implicants, visualize_bits};

fn main() {
    let ones = vec![
        vec![0, 0, 0, 0], //
        vec![0, 0, 1, 0], //
        vec![0, 1, 0, 0], //
        vec![0, 1, 0, 1], //
        vec![0, 1, 1, 0], //
        vec![1, 0, 1, 0], //
        vec![0, 1, 1, 1], //
        vec![1, 0, 1, 1], //
    ];

    let ones = parse(&ones);

    let primes = search_prime_implicants(ones);

    visualize_bits(&primes);
}
