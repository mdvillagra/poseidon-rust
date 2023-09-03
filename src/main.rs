extern crate poseidon_lib;


use ark_ff::{Field, PrimeField, FpConfig, BigInteger};
use ark_bls12_381::{Fr as F, FrConfig};
use ark_std::{One, Zero, UniformRand};

fn main() {
  let constants = poseidon_lib::read_constants("constants2.txt");

  println!("Number of round constants {}", constants.c.len());
  let modulus = <F as PrimeField>::MODULUS;
  println!("The modulus is 0x{:X}", modulus);

  let mut rng = ark_std::test_rng();
  let a = F::rand(&mut rng);
  let b = F::from(5);
  println!("0x{:X}",b.into_bigint());
}

