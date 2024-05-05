use std::env;
use std::io::{self, Write};
use std::fs::{File, OpenOptions};

fn write_u128_vec_to_file(file: &mut File, data: &[u128]) -> io::Result<()> {
    for (i, &num) in data.iter().enumerate() {
        if i > 0 {
            file.write_all(b" -> ")?;
        }
        // Convert each u128 to its byte representation
        let bytes = num.to_string().into_bytes();
        // Write the bytes to the file
        file.write_all(&bytes)?;
    }
    // Write a newline character after writing the sequence
    file.write_all(b"\n")?;
    Ok(())
}

fn run_through_sequence(a: u128) -> Vec<u128> {
    let mut b: Vec<u128> = Vec::new();
    let mut seq: u128 = a;
    while seq != 1 {
        if seq % 2 == 0 {
            seq /= 2;
        } else {
            seq = seq * 3 + 1;
        }
        b.push(seq);
    }
    b.insert(0, a); // Insert the initial number at the beginning of the sequence
    b
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        panic!("No output file provided (Correct usage file lower_bound upper_bound)");
    }
    if args.len() < 3
    {
        panic!("No bounds provided (Correct usage file lower_bound upper_bound)")
    }
    let lower_bound : usize = match args[1].parse()
    {
        Ok(n) => n,
        Err(_) => {panic!("Could not resolve lower_bound");}
    };
    let upper_bound : usize = match args[2].parse()
    {
        Ok(n) => n,
        Err(_) => {panic!("Could not resolve upper_bound");}
    };
    let file_str = &args[0];
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_str)?;

    let mut current_num: u128 = 1;
    for _ in lower_bound..upper_bound {
        let current_sequence = run_through_sequence(current_num);
        write_u128_vec_to_file(&mut file, &current_sequence)?;
        current_num += 1;
    }
    Ok(())
}
