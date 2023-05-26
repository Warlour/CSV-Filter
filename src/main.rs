extern crate csv;
extern crate clap;

use std::error::Error;
use std::fs::File;
use clap::{Arg, App};

fn main() {
    let matches = App::new("CSV Filter")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Sets the input CSV file")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("keywords")
            .short("k")
            .long("keywords")
            .value_name("KEYWORDS")
            .help("Sets the keywords to filter")
            .required(true)
            .takes_value(true))
        .get_matches();

    let filename = matches.value_of("input").unwrap();
    let keywords = matches.value_of("keywords").unwrap();

    if let Err(e) = read_csv_file(filename, keywords) {
        println!("Error: {}", e);
    }
}

fn read_csv_file(filename: &str, keywords: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let keyword_list: Vec<&str> = keywords.split(';').collect();

    for result in rdr.records() {
        let record = result?;
        let text = record.get(0).unwrap_or("");

        if keyword_list.iter().any(|&keyword| text.contains(keyword)) {
            println!("{}", text);
        }
    }

    Ok(())
}
