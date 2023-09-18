extern crate poseidon_lib;
use ark_bls12_381::Fq as Fqbls12_381;
use ark_bls12_381::Fr as Frbls12_381;
use ark_bls12_381::Fq as Fqbls12_377;
use ark_bls12_377::Fr as Frbls12_377;
use ark_ff::PrimeField;
use ark_std::UniformRand;



fn main() {
    let constants =
        poseidon_lib::read_constants::read_constants_bls12381_Fr_n255_t5_alpha5_M128_RF8_RP56();
    let mut rng = ark_std::test_rng();
    let mut input: Vec<Frbls12_381> = Vec::new();

    for _i in 0..100 {
        input.push(Frbls12_381::rand(&mut rng));
    }

    let output = poseidon_lib::hash(&input, &constants, 3, 3);
    println!("length is {}", output.len());
    for c in output.into_iter().map(|elem| elem.to_string()) {
        println!("{c}");
    }

    println!("Modulus of Fq of BLS12_381 is {}", <Fqbls12_381 as PrimeField>::MODULUS);
    println!("Modulus of Fr of BLS12_381 is {}", <Frbls12_381 as PrimeField>::MODULUS);
    println!("Modulus of Fq of BLS12_377 is {}", <Fqbls12_377 as PrimeField>::MODULUS);
    println!("Modulus of Fr of BLS12_377 is {}", <Frbls12_377 as PrimeField>::MODULUS);
}
