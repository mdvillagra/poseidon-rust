use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use ark_bls12_381::Fq;
use ark_ff::BigInt as arkBigInt;
use ark_ff::{BigInteger, FpConfig, PrimeField};
use ark_std::str::FromStr;
use core::str;

use num_bigint::BigInt;
use num_traits::Num;

pub struct Constants<T: PrimeField> {
    pub c: Vec<T>,           //round constants
    pub m: Vec<Vec<T>>,      //MDS matrix
    pub t: usize,            //width of the state
    pub partial_rounds: u32, //number of partial rounds
    pub full_rounds: u32,    //number of full rounds
    pub alpha: u32,          //power of the S-box
}

/*********************************************************
Executes de linear layer.
Multiplies the MDS matrix times the state
*********************************************************/
pub fn linear_layer<T: PrimeField>(state: &mut Vec<T>, constants: Constants<T>) {
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
*********************************************************/
pub fn sbox<T: PrimeField>(state: &mut Vec<T>, constants: Constants<T>, full: bool) {
    if full {
        // apply full s-box
        for i in 0..state.len() {
            let p: arkBigInt<1> = arkBigInt::from(constants.alpha);
            state[i] = state[i].pow(p);
        }
    } else {
        // apply partial s-box
        let p: arkBigInt<1> = arkBigInt::from(constants.alpha);
        state[0] = state[0].pow(p);
    }
}

/*********************************************************
Executes the ARK stage. Deletes each round constant
that gets multiplied.
*********************************************************/
pub fn ark<T: PrimeField>(state: &mut Vec<T>, constants: &mut Constants<T>) {
    for i in 0..state.len() {
        state[i].add_assign(constants.c[0]);
        constants.c.remove(0);
    }
}

/*********************************************************
Initialize a state vector
**********************************************************/
pub fn init_state<T: PrimeField>(state: &mut Vec<T>, t: usize) {
    state.clear();
    for _i in 0..t {
        state.push(T::ZERO);
    }
}

/********************************************************
Reads the round constants and MDS matriz from the
given file_name generated by the sage subrutine.
This function read files in the output format of
https://extgit.iaik.tugraz.at/krypto/hadeshash/-/blob/master/code/generate_parameters_grain.sage
 *********************************************************/
pub fn read_constants(file_name: &str) -> Constants<Fq> {
    /*
    # GF(p), alpha=5, N = 1275, n = 255, t = 5, R_F = 8, R_P = 60: sage generate_parameters_grain.sage 1 0 255 5 8 60 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
     */
    let file = File::open(file_name).expect("file not found");
    let reader = BufReader::new(file);

    let mut c: Vec<Fq> = Vec::new();
    let mut m: Vec<Vec<Fq>> = Vec::new();

    let mut i = 0;

    for line in reader.lines() {
        // line 2 contains the round constants
        if i == 2 {
            let mut rconst: String = line.unwrap().replace(" ", "").replace("'", "");
            rconst.pop();
            rconst.remove(0);

            let constants: Vec<&str> = rconst.split(',').collect();
            for constant in constants {
                //all constants in the file are writen in hex and need to be converted to dec
                let n = BigInt::from_str_radix(&constant[2..], 16).unwrap();
                let number: Fq = Fq::from_str(&n.to_string()).unwrap();
                c.push(number);
            }
            i += 1;
        }
        // line 15 contains the mds matrix
        else if i == 15 {
            let mut mds = line.unwrap().replace(" ", "").replace("'", "");
            mds.pop();
            mds.pop();
            mds.remove(0);
            mds.remove(0);
            let rows: Vec<&str> = mds.split("],[").collect();

            for r in rows {
                let rows_vector: Vec<&str> = r.split(",").collect();
                let mut mi: Vec<Fq> = Vec::new();
                for r2 in rows_vector {
                    //all constants in the file are writen in hex and need to be converted to dec
                    let n2 = BigInt::from_str_radix(&r2[2..], 16).unwrap();
                    let v2: Fq = Fq::from_str(&n2.to_string()).unwrap();
                    mi.push(v2);
                }
                m.push(mi);
            }
            i += 1;
        }
        i += 1;
    }

    Constants {
        c,
        m,
        t: 5,
        partial_rounds: 60,
        full_rounds: 8,
        alpha: 5,
    }
}
