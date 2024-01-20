//! A very simple CSV File creation, parsing and modification library.
//!
//! Provides an utility to create, modify, parse and save CSV files.

use crate::csv::CSVBuilder;
use crate::csv::CSVFile;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod csv;

/// Create a new empty CSV File
pub fn new_csv_builder() -> CSVBuilder {
    CSVBuilder::new()
}

/// Parse the file with the specified CSVBuilder
pub fn parse_from_file_with(
    path_to_file: &str,
    builder: CSVBuilder,
) -> Result<CSVFile, std::io::Error> {
    let reader = BufReader::new(File::open(path_to_file)?);
    parse(reader, builder)
}

/// Parse the file to a CSV File
pub fn parse_from_file(path_to_file: &str, has_header: bool) -> Result<CSVFile, std::io::Error> {
    let reader = BufReader::new(File::open(path_to_file)?);
    parse_from_reader(reader, has_header)
}

/// Parse the reader to a CSV File
pub fn parse_from_reader(
    reader: BufReader<File>,
    has_header: bool,
) -> Result<CSVFile, std::io::Error> {
    let csv_builder = CSVBuilder::new().has_header(has_header);

    parse(reader, csv_builder)
}

/// Parse the reader with the specified CSVBuilder
pub fn parse_from_reader_with(
    reader: BufReader<File>,
    builder: CSVBuilder,
) -> Result<CSVFile, std::io::Error> {
    parse(reader, builder)
}

fn parse(reader: BufReader<File>, mut csv_builder: CSVBuilder) -> Result<CSVFile, std::io::Error> {
    // Open file in read mode read file contents by line
    // If first line is a header, then parse it into header
    let mut lines = reader.lines();
    if csv_builder.has_header {
        let first_line = lines
            .next()
            .expect("Error reading first line from file")
            .expect("Error reading first line from file");
        csv_builder = csv_builder.header(first_line);
    }

    for (_index, line) in lines.flatten().enumerate() {
        csv_builder = csv_builder.row(line);
    }

    Ok(csv_builder.build())
}
