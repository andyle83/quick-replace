use text_colorizer::*;
use std::env;
use std::fs;

fn main() {
    let args = parse_args();
    println!("{:?}", args)

    // read file
    let data = match fs::read_to_string(&args.fileNameInput) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}", "Error".red().bold(), args.fileNameInput, e);
            std::process::exit(1);
        }
    };

    // write file
    match fs::write(&args.fileNameOutput, &data) {
        OK(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error".red().bold(), args.fileNameOutput, e);
            std::process::exit(1);
        }
    }


}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    fileNameInput: String,
    fileNameOutput: String,
}

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another", "quick-replacement".green());
    eprintln!("Usage: quick-replacement <target> <replacement> <INPUT> <OUTPUT>")
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expect 4, got {}.", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        fileNameInput: args[2].clone(),
        fileNameOutput: args[3].clone()
    }
}
