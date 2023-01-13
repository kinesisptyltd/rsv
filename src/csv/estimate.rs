use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // read 20000 lines to estimate bytes per line
    let file = File::open(path)?;
    let filesize = file.metadata()?.len() as f64;
    
    let mut total_bytes = 0;
    let mut n = 0;
    for l in BufReader::new(file).lines().skip(1) {
        total_bytes += l.unwrap().len() + 1;
        n += 1;

        if n > 20000 {
            break;
        }
    }

    // estimate line count
    let estimate_n = filesize / ((total_bytes as f64) / (n as f64));

    println!("{}", estimate_n as usize);
    Ok(())
}