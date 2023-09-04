extern crate poseidon_lib;


use ark_ff::{Field, PrimeField, FpConfig, BigInteger};
use ark_bls12_381::{Fq, FrConfig};
use ark_std::{One, Zero, UniformRand};

use num_bigint::BigInt;
use num_traits::Num;

fn main() {
  
  let constants = poseidon_lib::read_constants("constants2.txt");

  let mut state: Vec<Fq> = Vec::new();
  poseidon_lib::init_state(&mut state, 2);

  println!("Number of round constants {}", constants.c.len());
  let modulus = <Fq as PrimeField>::MODULUS;
  println!("The modulus is 0x{:X}", modulus);

  let mut rng = ark_std::test_rng();
  let a = Fq::rand(&mut rng);
  let b = Fq::from(5);
  println!("0x{:X}",b.into_bigint());
}

