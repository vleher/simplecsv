use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

#[derive(Debug)]
pub struct CSVFile {
    pub header: Vec<String>,
    pub data: Vec<Vec<String>>, // TODO: Data should support more than String type
}

impl CSVFile {
    pub fn get_value_by_name(&self, header_name: &String, row_index: usize) -> Option<String> {
        let col_index = self.header.iter().position(|x| x.eq(header_name))?;
        self.get_value_by_index(row_index, col_index)
    }

    pub fn get_value_by_index(&self, row_index: usize, col_index: usize) -> Option<String> {
        let value = &self.data.get(row_index);
        match value {
            Some(v) => v.get(col_index).cloned(),
            None => None,
        }
    }

    pub fn set_value_by_index(
        &mut self,
        row_index: usize,
        col_index: usize,
        value: String,
    ) -> Result<bool, &str> {
        if self.get_value_by_index(row_index, col_index).is_none() {
            return Err("Out of bounds");
        }
        self.data[row_index][col_index] = value;
        Ok(true)
    }

    pub fn set_value_by_name(
        &mut self,
        row_index: usize,
        header_name: &String,
        value: String,
    ) -> Result<bool, &str> {
        if self.get_value_by_name(header_name, row_index).is_none() {
            return Err("Out of bounds");
        }
        let col_index = self.header.iter().position(|x| x.eq(header_name));
        match col_index {
            Some(i) => {
                self.data[row_index][i] = value;
                Ok(true)
            }
            None => Err("Out of bounds"),
        }
    }

    pub fn save_to_file(&self, file_name: &str) -> Result<bool, std::io::Error> {
        let file = File::create(file_name)?;
        let mut csv_writer = BufWriter::new(file);

        let header: String = self
            .header
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if i >= self.header.len() - 1 {
                    return s.to_owned();
                }
                s.to_owned() + ","
            })
            .collect();

        if header.len() > 1 {
            writeln!(&mut csv_writer, "{}", header)?;
        }

        for row in &self.data {
            let row_str: String = row
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    if i >= row.len() - 1 {
                        return s.to_owned();
                    }
                    s.to_owned() + ","
                })
                .collect();
            writeln!(&mut csv_writer, "{}", row_str)?;
        }

        Ok(true)
    }
}

#[derive(Default)]
pub struct CSVBuilder {
    pub(crate) header: String,
    pub(crate) rows: Vec<String>,
    pub(crate) separator: char,
    pub(crate) has_header: bool,
}

impl CSVBuilder {
    pub fn new() -> CSVBuilder {
        CSVBuilder {
            header: String::new(),
            rows: Vec::new(),
            separator: ',',
            has_header: true,
        }
    }

    pub fn separator(mut self, separator: char) -> CSVBuilder {
        self.separator = separator;
        self
    }

    pub fn has_header(mut self, has_header: bool) -> CSVBuilder {
        self.has_header = has_header;
        self
    }

    pub fn header(mut self, header: String) -> CSVBuilder {
        self.header = header;
        self
    }

    pub fn row(mut self, row: String) -> CSVBuilder {
        self.rows.push(row);
        self
    }

    fn parse_from_string(input: &str) -> Vec<String> {
        if input.is_empty() {
            return Vec::new();
        }

        let fields = input.split(',').map(|s| s.to_string()).collect();
        fields
    }

    pub fn build(self) -> CSVFile {
        CSVFile {
            header: Self::parse_from_string(&self.header),
            data: {
                let mut v = Vec::new();
                for r in self.rows {
                    v.push(Self::parse_from_string(&r))
                }
                v
            },
        }
    }
}
