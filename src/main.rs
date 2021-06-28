use clap::{Arg, App};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process;
use std::error::Error;

fn main() {
    let matches = App::new("dict")
        .version("0.1.0")
        .author("sunguanke <sunguanke@live.com>")
        .about("ecdict")
        .subcommand(
            App::new("init")
            .about("initialize database")
            .version("0.1.0")
            .author("sunguanke <sunguanke@live.com>")
                .arg(Arg::new("file")
                    .about("the file of dict")
                    .required(true)
                    .index(1)))
        .arg(Arg::new("word")
            .about("the word to translate")
            .required(false)
            .index(1))
        .get_matches();
    if let Some(word) = matches.value_of("word") {
        println!("{}", word);
    }

    if let Some(ref matches) = matches.subcommand_matches("init") {
        if let Some(filename) = matches.value_of("file") {
            if let Err(err) = read_file(filename) {
                eprintln!("{}", err.to_string());
                process::exit(-1);
            }
        }
    }
}

fn read_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let file: File = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    let mut num_read = reader.read_line(&mut buf)?;
    while num_read != 0 {
        buf.clear();
        num_read = reader.read_line(&mut buf)?;
        let fileds: Vec<&str> = buf.split(',').collect();
        if num_read == 0 {
            break;
        }
    }

    Ok(())
}