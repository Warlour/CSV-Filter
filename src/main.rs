extern crate csv;
extern crate clap;

use std::error::Error;
use std::fs::File;
use clap::{Arg, App};

fn main() {
    let matches: clap::ArgMatches = App::new("CSV Filter")
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

    let filename: &str = matches.value_of("input").unwrap();
    let keywords: &str = matches.value_of("keywords").unwrap();

    if let Err(e) = read_csv_file(filename, keywords) {
        println!("Error: {}", e);
    }
}

fn read_csv_file(filename: &str, keywords: &str) -> Result<(), Box<dyn Error>> {
    let file: File = File::open(filename)?;
    let mut rdr: csv::Reader<File> = csv::Reader::from_reader(file);

    let keyword_list: Vec<&str> = keywords.split(';').collect();

    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        let text: String = record.get(0).unwrap_or("").to_owned();
        let lowercase_text: String = text.to_lowercase();

        let mut is_match: bool = true;

        for keyword in &keyword_list {
            let (prefix, keyword) = parse_keyword(keyword);
            let lowercase_keyword: String = keyword.to_lowercase();

            if prefix == "-" {
                if lowercase_text.contains(&lowercase_keyword) {
                    is_match = false;
                    break;
                }
            } else {
                if !lowercase_text.contains(&lowercase_keyword) {
                    is_match = false;
                    break;
                }
            }
        }

        if is_match {
            println!("{}", text);
        }
    }

    Ok(())
}

fn parse_keyword(keyword: &str) -> (&str, &str) {
    if keyword.starts_with('\\') {
        ("", &keyword[1..])
    } else if keyword.starts_with('-') {
        ("-", &keyword[1..])
    } else {
        ("", keyword)
    }
}
