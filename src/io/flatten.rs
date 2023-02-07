use crate::utils::{cli_result::CliResult, table::Table};
use std::io::{stdin, BufRead};

pub fn run(no_header: bool, sep: &str, delimiter: &str, n: i32) -> CliResult {
    let n = if n <= 0 { usize::MAX - 10 } else { n as usize };

    // open file and header
    let lines = stdin()
        .lock()
        .lines()
        .take(n + 1 - no_header as usize)
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();

    // too few rows
    if lines.len() <= 1 - no_header as usize {
        return Ok(());
    }

    let columns: Vec<String> = if no_header {
        (1..=lines[0].split(sep).count())
            .map(|i| format!("col{i}"))
            .collect::<Vec<_>>()
    } else {
        lines[0].split(sep).map(String::from).collect::<Vec<_>>()
    };

    // read file
    let mut rdr = lines.iter().skip(1 - no_header as usize).peekable();
    while let Some(l) = rdr.next() {
        let r = l
            .split(sep)
            .zip(&columns)
            .map(|(v, k)| vec![k.as_str(), v])
            .collect::<Vec<_>>();
        Table::from_records(r).print_blank()?;

        if rdr.peek().is_some() {
            println!(" {delimiter}");
        } else {
            println!();
        }
    }

    Ok(())
}
