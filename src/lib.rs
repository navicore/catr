use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ed Sweeney <ed@onextent.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .required(true)
                .multiple(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Number lines")
                .takes_value(false),
        )
        .get_matches();

    let sfiles: Vec<&str> = matches.values_of("files").unwrap().collect();
    let files: Vec<String> = sfiles.iter().map(|s| s.to_string()).collect();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");
    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
