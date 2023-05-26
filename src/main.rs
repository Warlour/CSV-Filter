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

    let filename = matches.value_of("input").unwrap();
    let keywords = matches.value_of("keywords").unwrap();

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
        let text: &str = record.get(0).unwrap_or("");

        let mut is_match: bool = true;
        let mut has_positive_requirement: bool = false;
        let mut has_negative_requirement: bool = false;

        for keyword in &keyword_list {
            let (prefix, keyword) = parse_keyword(keyword);

            if prefix == "+" {
                has_positive_requirement = true;
                if !text.contains(keyword) {
                    is_match = false;
                    break;
                }
            } else if prefix == "-" {
                has_negative_requirement = true;
                if text.contains(keyword) {
                    is_match = false;
                    break;
                }
            } else {
                if !text.contains(keyword) {
                    is_match = false;
                    break;
                }
            }
        }

        if is_match && (!has_positive_requirement || has_negative_requirement) {
            println!("{}", text);
        }
    }

    Ok(())
}

fn parse_keyword(keyword: &str) -> (&str, &str) {
    if keyword.starts_with('+') {
        ("+", &keyword[1..])
    } else if keyword.starts_with('-') {
        ("-", &keyword[1..])
    } else {
        ("", keyword)
    }
}
