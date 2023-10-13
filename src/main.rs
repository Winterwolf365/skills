mod book;

use chrono::prelude::*;
use clap::Parser;
use regex::Regex;
use serde::Serialize;
use std::fs;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Sort {
    Id,
    PublicationDate,
    Author,
    Genre,
    Title,
    TotalPages,
    Rating,
    Language,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Format {
    RustStruct,
    Json,
    Yaml,
}

#[derive(Debug, Clone, Serialize)]
pub struct Book<'a> {
    /// Unique identification number of the book
    id: u32,
    ///  Publication Date of the book (YYYY=year,MM=month,DD=day)
    publication_date: NaiveDate,
    /// Name of the author of the book
    author: &'a str,
    /// Genre of the book (e.g., Fic, Rom, SciFi)
    genre: &'a str,
    /// Title of the book
    title: &'a str,
    /// Number of pages in the book
    total_pages: u32,
    /// Rating of the book on a scale of 1.0-5.0
    rating: f64,
    /// Language in which the book is written (e.g., Nl, Dui, Eng, Spa, Dan, Rus, Fra, Ita, Pol, Rom, Cze, Por)
    language: &'a str,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Location of data file
    #[arg(short, long)]
    file: String,
    /// Sort the output
    #[arg(short, long, value_enum, default_value_t = Sort::Id)]
    sort: Sort,
    /// Format the output
    #[arg(short, long, value_enum, default_value_t = Format::RustStruct)]
    output: Format,
    /// Search for an specific id
    #[arg(short, long, default_value = None)]
    id: Option<u32>,
    /// Search for an author
    #[arg(short, long, default_value = None)]
    author: Option<String>,
    /// Search for an genre
    #[arg(short, long, default_value = None)]
    genre: Option<String>,
    /// Search for an title
    #[arg(short, long, default_value = None)]
    title: Option<String>,
    /// Search for an language
    #[arg(short, long, default_value = None)]
    language: Option<String>,
}

fn main() {
    let args = Args::parse();

    let file = fs::read_to_string(&args.file)
        .unwrap_or_else(|_| panic!("Unable to read file: '{}'", &args.file));

    let mut books: Vec<Book> = Vec::new();

    // in the data file there is first some information
    // than an # sign and than the actual data
    let file: Vec<&str> = file.split('#').collect();
    let _info = file[0];
    let data = file[1];

    for (index, data) in data.lines().enumerate() {
        if index == 0 || data.trim() == "" {
            continue;
        }

        let mut line: Vec<&str> = data.split(',').map(|line| line.trim()).collect();

        let temp_line: Vec<&str> = line.to_vec();

        // there are suppost be be 8 values in an line
        if line.len() < 8 {
            for (index, value) in temp_line.iter().enumerate() {
                if !value.contains("  ") {
                    continue;
                }

                let regex = Regex::new(r"\s{2,}").unwrap();
                let lines: Vec<&str> = regex.split(value).collect();

                for (i, value) in lines.iter().enumerate() {
                    if i == 0 {
                        line[index] = value
                    } else {
                        line.insert(index + i, value)
                    }
                }
            }
        }

        // the line varaiable should contain 8 values
        books.push(Book {
            id: parse_number(line[0]),
            publication_date: NaiveDate::parse_from_str(
                parse_number::<u32>(line[1]).to_string().as_str(),
                "%Y%m%d",
            )
            .unwrap(),
            author: line[2].trim(),
            genre: line[3].trim(),
            title: line[4].trim(),
            total_pages: parse_number(line[5]),
            // the valid range of ratings is 0.0 - 5.0
            rating: parse_number::<f64>(line[6]).clamp(0.0, 5.0),
            language: line[7].trim(),
        });
    }

    let books = book::filer_books(args.clone(), books);
    let books = book::sort_books(args.sort.clone(), books);
    let books = book::format_books(args.output.clone(), books);

    println!("{}", books);
}

fn parse_number<T: std::str::FromStr>(number: &str) -> T {
    let number: String = number
        .trim()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    if let Ok(number) = number.parse::<T>() {
        number
    } else {
        panic!(
            "\"{}\" is not of type {}",
            number.trim(),
            std::any::type_name::<T>()
        );
    }
}
