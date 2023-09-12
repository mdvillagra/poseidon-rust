pub mod read_constants;

use read_constants::*;

use ark_bls12_381::Fq as Fbls12_381;
use ark_ff::BigInt as arkBigInt;
use ark_ff::PrimeField;

/*********************************************************
Hashing function
*********************************************************/
pub fn hash<T: PrimeField>(
    input: &Vec<T>,
    constants: &Constants<T>,
    output_length: u32,
    r: usize,
) -> Vec<T> {
    let mut state = absorb(input, constants, r);
    let output = squeeze(&mut state, constants, output_length, r);
    output.clone()
}

/*********************************************************
Squeezing stage
*********************************************************/
fn squeeze<T: PrimeField>(
    state: &mut Vec<T>,
    constants: &Constants<T>,
    output_length: u32,
    r: usize,
) -> Vec<T> {
    let mut output: Vec<T> = Vec::new();

    while (output.len() as u32) < output_length {
        output.extend_from_slice(&state[..r]);
        poseidon_permutation(state, constants);
    }
    if output_length > 1 {
        while output.len() as u32 % output_length != 0 {
            output.pop();
        }
    } else {
        while (output.len() as u32) > 1 {
            output.pop();
        }
    }
    output.clone()
}

/*********************************************************
Absorbing stage
*********************************************************/
fn absorb<T: PrimeField>(input: &Vec<T>, constants: &Constants<T>, r: usize) -> Vec<T> {
    let mut state: Vec<T> = Vec::new();
    let padded_input = pad(input, r as u32);

    init_state(&mut state, constants.t);

    for i in (0..padded_input.len()).step_by(r) {
        add_block(&padded_input[i..i + r], &mut state, r);
        poseidon_permutation(&mut state, constants);
    }
    state.clone()
}

/*********************************************************
Add the inner state with the input slice
*********************************************************/
fn add_block<T: PrimeField>(input: &[T], state: &mut Vec<T>, r: usize) {
    for i in 0..r {
        state[i].add_assign(input[i]);
    }
}

/*********************************************************
Padding function for an input vector.
The functions pads input with 0s and returns a vector
that is a multiple of r. If the length of the input is a
multiple of r, then no padding takes place.
*********************************************************/
fn pad<T: PrimeField>(input: &Vec<T>, r: u32) -> Vec<T> {
    let mut padded_input: Vec<T> = input.to_vec();

    while padded_input.len() as u32 % r != 0 {
        padded_input.push(T::ZERO);
    }

    padded_input
}

/*********************************************************
Implements the poseidon permutation.
*********************************************************/
pub fn poseidon_permutation<T: PrimeField>(state: &mut Vec<T>, constants: &Constants<T>) {
    for i in 0..(constants.full_rounds + constants.partial_rounds) as usize {
        ark(state, constants, i);
        sbox(state, constants, i);
        linear_layer(state, constants);
    }
}

/*********************************************************
Executes de linear layer.
Multiplies the MDS matrix times the state
*********************************************************/
fn linear_layer<T: PrimeField>(state: &mut Vec<T>, constants: &Constants<T>) {
    let mut result: Vec<T> = Vec::new();
    init_state(&mut result, constants.t);

    for i in 0..constants.t {
        for j in 0..constants.t {
            result[i].add_assign(state[j] * constants.m[i][j]);
        }
    }
    *state = result.clone();
}

/*********************************************************
Executes the S-box stage
Computes for each element in the state x^alpha
The rounds are counted starting from 0.
*********************************************************/
fn sbox<T: PrimeField>(state: &mut Vec<T>, constants: &Constants<T>, round_number: usize) {
    if round_number as u32 >= constants.full_rounds / 2
        && (round_number as u32) < constants.full_rounds / 2 + constants.partial_rounds
    {
        // apply partial s-box
        let p: arkBigInt<1> = arkBigInt::from(constants.alpha);
        state[0] = state[0].pow(p);
    } else {
        // apply full s-box
        for i in 0..state.len() {
            let p: arkBigInt<1> = arkBigInt::from(constants.alpha);
            state[i] = state[i].pow(p);
        }
    }
}

/*********************************************************
Executes the ARK stage.
The rounds are counted starting from 0.
*********************************************************/
fn ark<T: PrimeField>(state: &mut Vec<T>, constants: &Constants<T>, round_number: usize) {
    for i in 0..constants.t {
        state[i].add_assign(constants.c[constants.t * round_number + i]);
    }
}

/*********************************************************
Initialize a state vector
**********************************************************/
fn init_state<T: PrimeField>(state: &mut Vec<T>, t: usize) {
    state.clear();
    for _i in 0..t {
        state.push(T::ZERO);
    }
}

/********************************************************
Tests
 *********************************************************/
#[cfg(test)]
mod poseidon_permutation {
    use crate::*;
    use ark_std::UniformRand;

    #[test]
    fn read_constants_files() {
        let constant = read_constants_bls12381_n255_t5_alpha5_M128_RF8_RP56();
        assert_eq!(
            (constant.partial_rounds + constant.full_rounds) * constant.t as u32,
            constant.c.len() as u32
        );
        assert_eq!(5, constant.m.len());
        assert_eq!(5, constant.m[0].len());
    }

    #[test]
    fn padd_test() {
        let state: Vec<Fbls12_381> = vec![
            Fbls12_381::from(1),
            Fbls12_381::from(2),
            Fbls12_381::from(3),
            Fbls12_381::from(4),
            Fbls12_381::from(5),
            Fbls12_381::from(6),
            Fbls12_381::from(7),
            Fbls12_381::from(8),
        ];

        let new_state = pad(&state, 3);

        assert_eq!(new_state.len(), 9);
    }

    #[test]
    fn ark_test() {
        let mut constants = read_constants_bls12381_n255_t5_alpha5_M128_RF8_RP56();
        let mut state: Vec<Fbls12_381> = Vec::new();
        let mut result: Vec<Fbls12_381> = Vec::new();
        let mut rng = ark_std::test_rng();

        constants.c.clear();

        for i in 0..constants.t {
            state.push(Fbls12_381::rand(&mut rng));
            constants.c.push(Fbls12_381::rand(&mut rng));
            result.push(state[i] + constants.c[i]);
        }

        ark(&mut state, &constants, 0);
        assert_eq!(state, result);
    }
}
