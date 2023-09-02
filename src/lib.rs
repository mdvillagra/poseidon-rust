use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::u64;

pub struct Constants {
    pub c: Vec<u64>,
    pub m: Vec<Vec<u64>>
}

pub fn read_constants(file_name: &str) -> Constants {
    let file = File::open(file_name).expect("file not found");
    let reader = BufReader::new(file);

    let mut c: Vec<u64> = Vec::new();
    let mut m: Vec<Vec<u64>> = Vec::new();

    let mut i = 0;

    for line in reader.lines() {
        if i == 2 {
            println!("Round constants\n");
            let mut rconst: String = line
                .unwrap()
                .replace(" ", "")
                .replace("'", "");
            rconst.pop();
            rconst.remove(0);
            //println!("{}\n", rconst);
            let constants: Vec<&str> = rconst.split(',').collect();
            for constant in constants {
                c.push(u64::from_str_radix(&constant[2..], 16).unwrap());
            }
            i += 1;
        }
        else if i == 15 {
            println!("MDS\n");
            let mut mds = line
                .unwrap()
                .replace(" ","")
                .replace("'", "");
            mds.pop(); mds.pop();
            mds.remove(0); mds.remove(0);
            let rows:  Vec<&str> = mds.split("],[").collect();
            //println!("{}\n", mds);
            for r in rows {
                let rows_vector: Vec<&str> = r.split(",").collect();
                let mut mi: Vec<u64> = Vec::new();
                for r2 in rows_vector {
                    let v2 = u64::from_str_radix(&r2[2..], 16).unwrap();
                    mi.push(v2);
                }
                m.push(mi);
            }
            i += 1;
        }
        i += 1;
    }
   
    Constants {c,m}

}