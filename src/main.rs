mod book;

use chrono::prelude::*;
use clap::Parser;
use colored::*;
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
    Debug,
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
    /// Read the data from a file location
    #[arg(short, long)]
    read: String,
    /// Sort the output
    #[arg(short, long, value_enum, default_value_t = Sort::Id)]
    sort: Sort,
    /// Format the output
    #[arg(short, long, value_enum, default_value_t = Format::Debug)]
    format: Format,
    /// Write the output to a file location
    #[arg(short, long, default_value = None)]
    write: Option<String>,
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
    let file = fs::read_to_string(&args.read).unwrap_or_else(|error| {
        println!(
            "{} unable to read file: \"{}\" \n{}",
            "error:".red().bold(),
            &args.read,
            error
        );

        std::process::exit(0);
    });
    let mut books: Vec<Book> = Vec::new();

    // in the data file there is first some information
    // than an # sign and than the actual data
    let file: Vec<&str> = file.split('#').collect();
    let info = file[0];
    let data = file[1];

    for (mut index, data) in data.lines().enumerate() {
        if index == 0 || data.trim() == "" {
            continue;
        }

        index += info.lines().count() + 1;
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
        if line.len() < 8 {
            println!(
                "{} expected 8 variables sperated by one comma or at least two spaces but found {} variables \n{} {}",
                "error:".red().bold(),
                line.len(),
                format!("{} |", index).cyan(),
                data
            );

            std::process::exit(0);
        }

        let publication_date = NaiveDate::parse_from_str(
            parse_number::<u32>(line[1], 2, index, data)
                .to_string()
                .as_str(),
            "%Y%m%d",
        )
        .unwrap_or_else(|_| {
            println!(
                "{} \"{}\"(variable no. 2) is not 8 numbers so it is not able to be parsed to an date \n{} {}",
                "error:".red().bold(),
                line[1],
                format!("{} |", index).cyan(),
                data,
            );

            std::process::exit(0);
        });

        books.push(Book {
            id: parse_number(line[0], 1, index, data),
            publication_date,
            author: line[2],
            genre: line[3],
            title: line[4],
            total_pages: parse_number(line[5], 6, index, data),
            // the valid range of ratings is 0.0 - 5.0
            rating: parse_number::<f64>(line[6], 7, index, data),
            language: line[7],
        });
    }

    let books = book::filer_books(args.clone(), books);
    let books = book::sort_books(args.sort.clone(), books);
    let books = book::format_books(args.format.clone(), books);

    if let Some(write) = &args.write {
        fs::write(write, &books).unwrap_or_else(|error| {
            println!(
                "{} unable to write to: \"{}\" \n{}",
                "error:".red().bold(),
                write,
                error,
            );

            std::process::exit(0);
        });
    }

    println!("{}", &books);
}

fn parse_number<T: std::str::FromStr>(number: &str, variable: u32, index: usize, line: &str) -> T {
    let filterd_number: String = number
        .chars()
        .filter(|c| c.is_ascii_digit() || c == &'.')
        .collect();

    if let Ok(filterd_number) = filterd_number.parse::<T>() {
        filterd_number
    } else {
        println!(
            "{} \"{}\"(variable no. {}) is not of type {} \n{} {}",
            "error:".red().bold(),
            number,
            variable,
            std::any::type_name::<T>(),
            format!("{} |", index).cyan(),
            line,
        );

        std::process::exit(0);
    }
}
