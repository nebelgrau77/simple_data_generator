//! usage: 
//! datagen [number of lines] [lower bound for previous/current] [upper bound, non-inclusive] [output file path]
//! 

use std::fs::File;
use std::io::{Write, BufWriter};
use rand::Rng;


fn main() {

    // get the number of lines to generate from arguments
    let number = std::env::args().nth(1).unwrap_or_else(|| 10.to_string());
    let lower = std::env::args().nth(2).unwrap_or_else(|| 0.to_string());
    let upper = std::env::args().nth(3).unwrap_or_else(|| 2.to_string());
    let write_path = std::env::args().nth(4).unwrap_or_else(|| "output.txt".to_string());

    // parse values from arguments
    let n = number.parse().unwrap_or(10);
    let lo = lower.parse().unwrap_or(0);
    let mut up = upper.parse().unwrap_or(2);

    if up <= lo {
        up = lo + 2;
        println!("upper bound must be higher than the lower bound. Upper bound set to {}.", up);
    }

    // set up writing to file
    let output = File::create(write_path).unwrap();
    let mut buffered_output = BufWriter::new(output);

    // get ASCII uppercase letters to generate 'names'
    const ASCII: &str  = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LENGTH: usize = ASCII.len();
    let mut arr: [char; LENGTH] = [' '; LENGTH];
    for (idx, letter) in ASCII.chars().enumerate() {
        arr[idx as usize] = letter;
    }
    
    let header = "index,name,previous,current\n";

    write!(buffered_output, "{}", header).unwrap();

    for a in 0..n {
        let previous = rand::thread_rng().gen_range(&lo,&up);
        let current = rand::thread_rng().gen_range(&lo,&up);

        let rand_idx: usize = rand::thread_rng().gen_range(0_usize,&LENGTH);
        let name = arr[rand_idx];

        let new_string = format!("{},{},{},{}\n",a,&name,&previous,&current);

        write!(buffered_output, "{}", new_string).unwrap();

        }

}


