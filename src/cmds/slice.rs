use crate::utils::filename::{full_path, new_path};

use crate::utils::file::file_or_stdout_wtr;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::{error::Error, io::BufReader};

pub fn run(
    filename: &str,
    no_header: bool,
    start: usize,
    end: Option<usize>,
    length: Option<usize>,
    index: Option<usize>,
    export: bool,
) -> Result<(), Box<dyn Error>> {
    // current file
    let path = full_path(filename)?;
    let out_path = new_path(&path, "-slice");

    // open file
    let f = file_or_stdout_wtr(export, &out_path)?;

    let mut wtr = BufWriter::new(f);
    let mut rdr = BufReader::new(File::open(&path)?);

    // header
    if !no_header {
        let mut buf = vec![];
        rdr.read_until(b'\n', &mut buf)?;
        wtr.write_all(&buf)?;
    }

    // slice
    match index {
        Some(index) => write_by_index(&mut rdr, &mut wtr, index)?,
        None => {
            let e = end
                .or_else(|| length.map(|l| start + l).or(Some(usize::MAX)))
                .unwrap();
            write_by_range(&mut rdr, &mut wtr, start, e)?;
        }
    }

    if export {
        println!("Saved to file: {}", out_path.display())
    }

    Ok(())
}

fn write_by_index(
    rdr: &mut BufReader<File>,
    wtr: &mut BufWriter<Box<dyn Write>>,
    index: usize,
) -> std::io::Result<()> {
    let mut buf = vec![];
    let mut n = 0;

    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 {
            break;
        }

        if n == index {
            wtr.write_all(&buf[..bytes])?;
            break;
        }

        buf.clear();
        n += 1;
    }

    Ok(())
}

fn write_by_range(
    rdr: &mut BufReader<File>,
    wtr: &mut BufWriter<Box<dyn Write>>,
    start: usize,
    end: usize,
) -> std::io::Result<()> {
    let mut buf = vec![];
    let mut n = 0;

    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 || n >= end {
            break;
        }

        if n >= start && n < end {
            wtr.write_all(&buf[..bytes])?;
        }

        buf.clear();
        n += 1;
    }

    Ok(())
}
