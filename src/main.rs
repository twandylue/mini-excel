mod tokens;

use std::env::{self, Args};
use std::process::ExitCode;

fn main() -> ExitCode {
    match entry() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("failed to get program name");
    let mut subcommand: Option<String> = None;
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                usage(&program);
                return Ok(());
            }
            _ => subcommand = Some(arg),
        }
    }

    let subcommand: String = subcommand.ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: No subcommand is specified.");
    })?;

    match subcommand.as_str() {
        "cal" => {
            let input_content = read_file(&mut args, &program)?;
            println!("Original Content:\n{input_content}");

            parse_csv(&input_content)?;
            println!("---------");
            let (row, col) = estimate_table_size(&input_content);
            println!("row x col: {row} x {col}");

            // cal(input_content)?;
        }
        _ => {
            eprintln!("ERROR: Unknown subcommand: {}", subcommand);
            usage(&program);
            return Err(());
        }
    }

    Ok(())
}

fn read_file(args: &mut Args, program: &str) -> Result<String, ()> {
    let input_csv_file = args.next().ok_or_else(|| {
        eprintln!("ERROR: No input csv file is specified.");
        usage(&program);
    })?;

    let input_content = std::fs::read_to_string(input_csv_file).map_err(|e| {
        eprintln!("ERROR: Failed to read input csv file: {}", e);
    })?;

    Ok(input_content)
}

fn estimate_table_size(input_content: &str) -> (usize, usize) {
    let mut row_count = 0;
    let mut col_count = 0;
    for line in input_content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        row_count += 1;
        let mut col_count_in_line = 0;
        for col in line.split('|') {
            col_count_in_line += 1;
        }
        if col_count_in_line > col_count {
            col_count = col_count_in_line;
        }
    }

    (row_count, col_count)
}

fn parse_csv(input_content: &str) -> Result<(), ()> {
    // TODO:
    for col in input_content.split('|') {
        println!("{col}", col = col.trim());
    }

    Ok(())
}

fn cal(input_content: String) -> Result<(), ()> {
    todo!();
}

fn usage(program: &str) {
    eprintln!("Usage: {program} [SBUCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("     cal <input-csv-file>         Calculate the input csv file.");
}
