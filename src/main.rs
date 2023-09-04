extern crate poseidon_lib;


use ark_ff::{Field, PrimeField, FpConfig, BigInteger};
use ark_bls12_381::{Fq, FrConfig};
use ark_std::{One, Zero, UniformRand};
use ark_ff::BigInt as arkBigInt;

use num_bigint::BigInt;
use num_traits::Num;

fn main() {
  
  let mut constants = poseidon_lib::read_constants("constants2.txt");

  let mut state: Vec<Fq> = Vec::new();
  poseidon_lib::init_state(&mut state, 2);

  poseidon_lib::ark(&mut state, &mut constants);

  println!("{}",state[0]);

  poseidon_lib::sbox(&mut state, constants, true);

  println!("{}",state[0]);
}

