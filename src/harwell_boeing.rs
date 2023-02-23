#![allow(dead_code)]

use std::{
    fs::File,
    io::{self, BufRead, Lines},
    str::FromStr,
};

use num::Num;

#[derive(Debug)]
enum MatrixValueType {
    Real,
    Complex,
    Pattern,
}

impl MatrixValueType {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'R' => Self::Real,
            'C' => Self::Complex,
            'P' => Self::Pattern,
            _ => panic!("Unexpected matrix value type character '{ch}'"),
        }
    }
}

#[derive(Debug)]
enum MatrixProperty {
    Unsymmetric,
    Symmetric,
    Hermitian,
    SkewSymmetric,
    Rectangular,
}

impl MatrixProperty {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'U' => Self::Unsymmetric,
            'S' => Self::Symmetric,
            'H' => Self::Hermitian,
            'Z' => Self::SkewSymmetric,
            'R' => Self::Rectangular,
            _ => panic!("Unexpected matrix property character '{ch}'"),
        }
    }
}

#[derive(Debug)]
enum MatrixStructure {
    Assembled,
    FiniteElement,
}

impl MatrixStructure {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'A' => Self::Assembled,
            'E' => Self::FiniteElement,
            _ => panic!("Unexpected matrix structure character '{ch}'"),
        }
    }
}

#[derive(Debug)]
struct MatrixType {
    value_type: MatrixValueType,
    property: MatrixProperty,
    structure: MatrixStructure,
}

impl MatrixType {
    pub fn from_string(string: String) -> Self {
        if string.len() != 3 {
            panic!("MatrixType string must have exactly three characters")
        }
        let value_type = MatrixValueType::from_char(string.chars().nth(0).unwrap());
        let property = MatrixProperty::from_char(string.chars().nth(1).unwrap());
        let structure = MatrixStructure::from_char(string.chars().nth(2).unwrap());
        MatrixType {
            value_type,
            property,
            structure,
        }
    }
}

/// Data contained in the header of a Harwell-Boeing
/// matrix file.
///
/// The Harwell-Boeing format is described
/// [here](https://people.sc.fsu.edu/~jburkardt/data/hb/hb.html).
///
#[derive(Debug)]
struct HarwellBoeingHeader {
    /// Title of matrix
    title: String,
    /// Matrix key (another identifier?)
    key: String,
    /// Total number of data lines
    total_data_lines: i32,
    /// Num + FromStrber of data lines for column offset
    num_column_offset_lines: i32,
    /// Num + FromStrber of lines for row indices
    num_row_index_lines: i32,
    /// Num + FromStrber of lines for non-zero values
    num_values_lines: i32,
    /// Num + FromStrber of lines for right-hand side,
    /// starting guess, and solutions
    num_rhs_lines: i32,
    /// Matrix type, as a three-character code
    matrix_type: MatrixType,
    /// Num + FromStrber of rows in the matrix
    num_rows: i32,
    /// Num + FromStrber of columns in the matrix
    num_columns: i32,
    /// Num + FromStrber of non-zero values in the matrix
    num_non_zeros: i32,
    num_elemental_entries: i32,
    pointer_format: String,
    index_format: String,
    value_format: String,
    rhs_format: String,
    rhs_type: Option<String>,
    /// Num + FromStrber of right-hand sides
    num_rhs: Option<i32>,
    num_rhs_indices: Option<i32>,
}

/// Sparse matrix stored in Harwell-Boeing format. Often, the
/// matrix may not contain right-hand side information (in this
/// case it stores a single sparse matrix in compressed-column
/// format). Sometimes, the non-zero values may be omitted, in
/// which case the matrix is called a "pattern matrix".
///
/// The Harwell-Boeing format uses arrays that are indexed from
/// one. These are converted to zero-indexed arrays in this struct.
///
#[derive(Debug)]
pub struct HarwellBoeingMatrix<P: Num + FromStr> {
    /// The header describing the matrix format
    header: HarwellBoeingHeader,
    /// Offsets to the start of each column in the row_indices vector
    column_offsets: Vec<i32>,
    /// Row indices for non-zero values in each column
    row_indices: Vec<i32>,
    /// Non-zero values corresponding to entries in row_indices
    non_zero_values: Option<Vec<P>>,
    /// Right-hand side, starting guess, initial solutions
    rhs_info: Option<Vec<P>>,
}

fn parse_int(buf: &str) -> i32 {
    buf[..14]
        .trim()
        .parse::<i32>()
        .expect("Failed to parse an integer in Harwell-Boeing file")
}

/// Fortran indexes arrays from 1, so subtract 1 here to
/// get indices suitable for using in SuperLU
fn parse_offset(buf: &str) -> i32 {
    buf.trim().parse::<i32>().expect("Failed to parse offset") - 1
}

fn parse_float<P: Num + FromStr>(buf: &str) -> P {
    match buf.trim().parse::<P>() {
        Ok(value) => value,
        Err(_) => panic!("Failed parsing floating-point value"),
    }
}

fn parse_vector<P: Num + FromStr>(
    lines: &mut Lines<io::BufReader<File>>,
    num_lines: i32,
    value_len: usize,
    value_parser: fn(&str) -> P,
) -> Vec<P> {
    let mut vector = Vec::new();
    for _ in 0..num_lines {
        let line = lines
            .next()
            .expect("Failed to read line while parsing Harwell-Boeing vector")
            .unwrap();
        assert!(
            line.len() % value_len == 0,
            "Values line length not a multiple of {value_len}"
        );
        for k in 0..line.len() / value_len {
            let value = value_parser(&line[value_len * k..][..value_len]);
            vector.push(value);
        }
    }
    vector
}

impl<P: Num + FromStr> HarwellBoeingMatrix<P> {
    pub fn num_columns(&self) -> usize {
        self.header.num_columns as usize
    }

    pub fn num_rows(&self) -> usize {
        self.header.num_rows as usize
    }

    pub fn to_vectors(self) -> (Vec<i32>, Vec<i32>, Vec<P>) {
        let non_zero_values = match self.non_zero_values {
            Some(non_zero_values) => non_zero_values,
            None => panic!("Missing non-zero values vector in matrix"),
        };
        (self.column_offsets, self.row_indices, non_zero_values)
    }

    pub fn from_file(file: std::fs::File) -> Self {
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();

        let line = lines
            .next()
            .expect("Expected at least 1 line in Harwell-Boeing file")
            .expect("Failed to parse line");
        let title = line[0..72].trim().to_string();
        let key = line[72..80].trim().to_string();

        let line = lines
            .next()
            .expect("Expected at least 2 line in Harwell-Boeing file")
            .expect("Failed to parse line");
        let total_data_lines = parse_int(&line[0 * 14..]);
        let num_column_offset_lines = parse_int(&line[1 * 14..]);
        let num_row_index_lines = parse_int(&line[2 * 14..]);
        let num_values_lines = parse_int(&line[3 * 14..]);
        let num_rhs_lines = parse_int(&line[4 * 14..]);

        let line = lines
            .next()
            .expect("Expected at least 3 line in Harwell-Boeing file")
            .expect("Failed to parse line");
        let matrix_type = MatrixType::from_string(line[0..3].trim().to_string());
        let num_rows = parse_int(&line[1 * 14..]);
        let num_columns = parse_int(&line[2 * 14..]);
        let num_non_zeros = parse_int(&line[3 * 14..]);
        let num_elemental_entries = parse_int(&line[4 * 14..]);

        let line = lines
            .next()
            .expect("Expected at least 4 line in Harwell-Boeing file")
            .expect("Failed to parse line");
        let pointer_format = line[0..16].trim().to_string();
        let index_format = line[16..32].trim().to_string();
        let value_format = line[32..52].trim().to_string();
        let rhs_format = line[52..72].trim().to_string();

        let (rhs_type, num_rhs, num_rhs_indices) = if num_rhs_lines > 0 {
            let line = lines
                .next()
                .expect("Expected at least 4 line in Harwell-Boeing file")
                .expect("Failed to parse line");
            let rhs_type = line[0..14].trim().to_string();
            let num_rhs = parse_int(&line[1 * 14..]);
            let num_rhs_indices = parse_int(&line[2 * 14..]);
            (Some(rhs_type), Some(num_rhs), Some(num_rhs_indices))
        } else {
            (None, None, None)
        };

        let header = HarwellBoeingHeader {
            title,
            key,
            total_data_lines,
            num_column_offset_lines,
            num_row_index_lines,
            num_values_lines,
            num_rhs_lines,
            matrix_type,
            num_rows,
            num_columns,
            num_non_zeros,
            num_elemental_entries,
            pointer_format,
            index_format,
            value_format,
            rhs_format,
            rhs_type,
            num_rhs,
            num_rhs_indices,
        };

        let column_offsets = parse_vector(&mut lines, num_column_offset_lines, 5, parse_offset);
        let row_indices = parse_vector(&mut lines, num_row_index_lines, 5, parse_offset);
        let non_zero_values = parse_vector(&mut lines, num_values_lines, 15, parse_float);

        println!("{:?}", header);
        println!("non zero values = {}", non_zero_values.len());
        println!("row indices = {}", row_indices.len());
        println!("col pointers = {}", column_offsets.len());

        Self {
            header,
            column_offsets,
            row_indices,
            non_zero_values: Some(non_zero_values),
            rhs_info: None,
        }
    }
}
