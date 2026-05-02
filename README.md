# poseidon-rust

[![Rust tests](https://github.com/mdvillagra/poseidon-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/mdvillagra/poseidon-rust/actions/workflows/rust.yml)

Rust implementation of the [Poseidon hash function](https://eprint.iacr.org/2019/458) over arkworks prime fields.

This repository is intended as a compact research and learning implementation: the permutation, sponge absorb/squeeze flow, parameter loading, and unit tests are visible in a small codebase.

## What is included

- Poseidon permutation with full and partial rounds.
- Sponge-style `hash` function with caller-provided rate and output length.
- BLS12-381 scalar-field parameter file for `n=255`, `t=5`, `alpha=5`, `M=128`, `R_F=8`, `R_P=56`.
- Round constants and MDS matrix generated with the Grain LFSR approach used by the HadesHash reference implementation.
- Unit tests for parameter loading, padding, and the ARK step.

## Usage

Add the repository as a path or Git dependency, then use the library crate name `poseidon_lib`.

```rust
use ark_bls12_381::Fr;
use poseidon_lib::{
    hash,
    read_constants::read_constants_bls12381_Fr_n255_t5_alpha5_M128_RF8_RP56,
};

let constants = read_constants_bls12381_Fr_n255_t5_alpha5_M128_RF8_RP56();
let input = vec![Fr::from(1_u64), Fr::from(2_u64), Fr::from(3_u64)];

// output_length = 1, rate = 3
let digest = hash(&input, &constants, 1, 3);
assert_eq!(digest.len(), 1);
```

## Running locally

```bash
cargo test
cargo run
```

`cargo run` prints a sample hash output and field moduli for BLS12-381 and BLS12-377.

## Correctness and security notes

- This is not an audited cryptographic library.
- The committed parameter file covers the BLS12-381 scalar field configuration above. Additional field/parameter loaders should be paired with committed parameter files and test vectors.
- For production use, compare outputs against independent Poseidon test vectors for the exact field, width, S-box exponent, round counts, and domain-separation convention you need.
