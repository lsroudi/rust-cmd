use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type CustomResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_blink_line: bool,
}

pub fn run(config: Config) -> CustomResult<()> {
    for filename in config.files {

        match open(&filename) {
            Err(e) => eprintln!("Failed to open {} : {}",filename,e),
            Ok(_) => println!("file opened {} ", filename)
        }
        //let content = open(&filename);
        println!("filename is {}",filename);
    }
    Ok(())
}

pub fn open(file:&str) -> CustomResult<Box<dyn BufRead>> {

    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
        
    }
}
pub fn get_args() -> CustomResult<Config> {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("El akroudi Abdessamad <lsroudi@gmail.com>")
        .about("Rust cat training")
        .arg(
            Arg::with_name("files") 
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number") 
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank") 
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

        Ok(Config { 
            files: matches.values_of_lossy("files").unwrap(),
            number_lines: matches.is_present("number"),
            number_blink_line: matches.is_present("number_nonblank")
        })
}
