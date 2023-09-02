extern crate poseidon_lib;

fn main() {
  let constants = poseidon_lib::read_constants("constants.txt");

  println!("Number of round constants {}", constants.c.len());

}

