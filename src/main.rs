extern crate poseidon_lib;
use ark_bls12_381::Fq as Fbls12_381;
use ark_std::UniformRand;

fn main() {
    let constants =
        poseidon_lib::read_constants::read_constants_bls12381_n255_t5_alpha5_M128_RF8_RP56();
    let mut rng = ark_std::test_rng();
    let mut input: Vec<Fbls12_381> = Vec::new();

    for _i in 0..100 {
        input.push(Fbls12_381::rand(&mut rng));
    }

    let output = poseidon_lib::hash(&input, &constants, 3, 3);
    println!("length is {}", output.len());
    for c in output.into_iter().map(|elem| elem.to_string()) {
        println!("{c}");
    }
}
