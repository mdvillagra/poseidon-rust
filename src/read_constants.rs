use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use ark_bls12_381::Fq as Fbls12_381;
use ark_ff::PrimeField;
use ark_std::str::FromStr;
use core::str;

use num_bigint::BigInt;
use num_traits::Num;

#[derive(Clone, Debug)]
pub struct Constants<T: PrimeField> {
    pub c: Vec<T>,           //round constants
    pub m: Vec<Vec<T>>,      //MDS matrix
    pub t: usize,            //width of the state
    pub partial_rounds: u32, //number of partial rounds
    pub full_rounds: u32,    //number of full rounds
    pub alpha: u32,          //exponent of the S-box
}

/********************************************************
Reads the round constants and MDS matriz from the
given file_name generated by the sage subrutine.
This function read files in the output format of
https://extgit.iaik.tugraz.at/krypto/hadeshash/-/blob/master/code/generate_params_poseidon.sage
 *********************************************************/
#[allow(non_snake_case)]
pub fn read_constants_bls12381_n255_t5_alpha5_M128_RF8_RP56() -> Constants<Fbls12_381> {
    /*
    Params: n=255, t=5, alpha=5, M=128, R_F=8, R_P=56
    Modulus = 52435875175126190479447740508185965837690552500527637822603658699938581184513
    Number of round constants: 320
     */
    let file = File::open("bls12_381_constants.txt").expect("file not found");
    let reader = BufReader::new(file);

    let mut c: Vec<Fbls12_381> = Vec::new();
    let mut m: Vec<Vec<Fbls12_381>> = Vec::new();

    let mut i = 0;

    for line in reader.lines() {
        // line 5 contains the round constants
        if i == 5 {
            let mut rconst: String = line.unwrap().replace(" ", "").replace("'", "");
            rconst.pop();
            rconst.remove(0);

            let constants: Vec<&str> = rconst.split(',').collect();
            for constant in constants {
                //all constants in the file are writen in hex and need to be converted to dec
                let n = BigInt::from_str_radix(&constant[2..], 16).unwrap();
                let number: Fbls12_381 = Fbls12_381::from_str(&n.to_string()).unwrap();
                c.push(number);
            }
            i += 1;
        }
        // line 18 contains the mds matrix
        else if i == 18 {
            let mut mds = line.unwrap().replace(" ", "").replace("'", "");
            mds.pop();
            mds.pop();
            mds.remove(0);
            mds.remove(0);
            let rows: Vec<&str> = mds.split("],[").collect();

            for r in rows {
                let rows_vector: Vec<&str> = r.split(",").collect();
                let mut mi: Vec<Fbls12_381> = Vec::new();
                for r2 in rows_vector {
                    //all constants in the file are writen in hex and need to be converted to dec
                    let n2 = BigInt::from_str_radix(&r2[2..], 16).unwrap();
                    let v2: Fbls12_381 = Fbls12_381::from_str(&n2.to_string()).unwrap();
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
        partial_rounds: 56,
        full_rounds: 8,
        alpha: 5,
    }
}