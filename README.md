# QMC – Quine-McCluskey Minimizer

This Rust project implements the Quine-McCluskey algorithm for minimizing Boolean functions. It provides functionality for parsing, grouping, comparing, and visualizing bit patterns (minterms) and identifies the prime implicants of a switching function.

## Example

```rust
use qmc::{parse, search_prime_implicants, visualize_bits};

fn main() {
    let ones = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 1, 0, 0],
        vec![0, 1, 0, 1],
        vec![0, 1, 1, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 1],
    ];

    let ones = parse(&ones);
    let primes = search_prime_implicants(ones);
    visualize_bits(&primes);
}
```

## Usage

1. **Clone & Build**
   ```sh
   git clone <repo-url>
   cd qmc
   cargo build
   ```

2. **Run**
   ```sh
   cargo run
   ```

3. **Run Tests**
   ```sh
   cargo test
   ```

## Project Structure

- `src/lib.rs` – Core logic and algorithms
- `src/main.rs` – Example application
- `tests/tests.rs` – Integration tests

## License

BSD3

---

**Note:**  
This project is a learning project and can be extended freely!