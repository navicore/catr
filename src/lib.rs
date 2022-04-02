use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ed Sweeney <ed@onextent.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .long("number-nonblank")
                .short("b")
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number-nonblank"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to print {}: {}", filename, err),
            Ok(f) => {
                let mut number = 0;
                for line in f.lines() {
                    match line {
                        Err(err) => eprintln!("Failed to print line: {}", err),
                        Ok(l) => {
                            if config.number_lines
                                || (config.number_nonblank_lines && !l.trim().is_empty())
                            {
                                number = number + 1;
                            }
                            let mut number_str = String::from("");
                            if config.number_lines
                                || (config.number_nonblank_lines && !l.trim().is_empty())
                            {
                                number_str = format!("     {}\t", number.to_string());
                            };
                            println!("{}{}", number_str, l)
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
