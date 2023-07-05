use std::env::{self, Args};
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("Hello, world!");
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
            "help" | "h" => {
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

            // TODO: cal
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

fn usage(program: &str) {
    eprintln!("Usage: {program} [SBUCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("     cal <input-csv-file>         Calculate the input csv file.");
}