use text_colorizer::*;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <input> <output>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.",
                  "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(str) => str,
        Err(err) => {
            eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), args.filename, err);
            std::process::exit(1);
        }
    };

    let data = match replace(&args.target, &args.replacement, &data) {
        Ok(str) => str,
        Err(err) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), err);
            std::process::exit(1);
        }
    };

    if let Err(err) = fs::write(&args.output, &data) {
        eprintln!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), args.output, err);
        std::process::exit(1);
    };
}
