//! usage: 
//! 
//! datagen [number of lines] [lower bound for previous/current] [upper bound, non-inclusive] [output file path]
//! 
//! TO DO: 
//! - check if upper > lower
//! 

use std::fs::File;
use std::io::{Write, BufWriter, BufReader, BufRead, Error};
use rand::Rng;


fn main() {

    // get the number of lines to generate from arguments
    let number = std::env::args().nth(1).unwrap_or(10.to_string());
    let lower = std::env::args().nth(2).unwrap_or(0.to_string());
    let upper = std::env::args().nth(3).unwrap_or(2.to_string());
    let write_path = std::env::args().nth(4).unwrap_or("output.txt".to_string());

    // parse values from arguments
    let n = number.parse().unwrap_or(10);
    let lo = lower.parse().unwrap_or(0);
    let up = upper.parse().unwrap_or(2);

    // set up writing to file
    let output = File::create(write_path).unwrap();
    let mut buffered_output = BufWriter::new(output);

    // get ASCII uppercase letters to generate 'names'
    const my_string: &'static str  = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const length: usize = my_string.len();
    let mut arr: [char; length] = [' '; length];
    for (idx, letter) in my_string.chars().enumerate() {
        arr[idx as usize] = letter;
    }

    let sep = ','.to_string();
    let end = '\n'.to_string();
    
    let header = "index,name,previous,current\n";

    write!(buffered_output, "{}", header).unwrap();

    for a in 0..n {
        let previous = rand::thread_rng().gen_range(&lo,&up);
        let current = rand::thread_rng().gen_range(&lo,&up);

        let rand_idx: usize = rand::thread_rng().gen_range(0_usize,&length);
        let name = arr[rand_idx];

        let new_string = a.to_string() + &sep + &name.to_string() + &sep + &previous.to_string() + &sep + &current.to_string() + &end;

        write!(buffered_output, "{}", new_string).unwrap();


        }
        
          

}


