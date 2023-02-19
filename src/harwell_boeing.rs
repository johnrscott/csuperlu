use std::{
    io::{self, BufRead},
    str::FromStr
};

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
	let value_type = MatrixValueType::from_char(string.chars().nth(0)
						    .unwrap());
	let property = MatrixProperty::from_char(string.chars().nth(1)
						 .unwrap());
	let structure = MatrixStructure::from_char(string.chars().nth(2)
						   .unwrap());
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
pub struct HarwellBoeingHeader {
    /// Title of matrix 
    title: String,
    /// Matrix key (another identifier?)
    key: String,
    /// Total number of data lines
    total_data_lines: i32,
    /// Number of data lines for column offset
    num_column_offset_lines: i32,
    /// Number of lines for row indices
    num_row_index_lines: i32,
    /// Number of lines for non-zero values
    num_values_lines: i32,
    /// Number of lines for right-hand side,
    /// starting guess, and solutions
    num_rhs_lines: i32,
    /// Matrix type, as a three-character code
    matrix_type: MatrixType,
    /// Number of rows in the matrix
    pub num_rows: i32,
    /// Number of columns in the matrix
    pub num_columns: i32,
    /// Number of non-zero values in the matrix
    pub num_non_zeros: i32,
    num_elemental_entries: i32,
    pointer_format: String,
    index_format: String,
    value_format: String,
    rhs_format: String,
    rhs_type: Option<String>,
    /// Number of right-hand sides
    num_rhs: Option<i32>,
    num_rhs_indices: Option<i32>,
}

/// Sparse matrix stored in Harwell-Boeing format. Often, the
/// matrix may not contain right-hand side information (in this
/// case it stores a single sparse matrix in compressed-column
/// format). Sometimes, the non-zero values may be omitted, in
/// which case the matrix is called a "pattern matrix".
///
#[derive(Debug)]
pub struct HarwellBoeingMatrix<P: FromStr> {
    /// The header describing the matrix format
    pub header: HarwellBoeingHeader,
    /// Offsets to the start of each column in the row_indices vector
    pub column_offsets: Vec<i32>,
    /// Row indices for non-zero values in each column
    pub row_indices: Vec<i32>,
    /// Non-zero values corresponding to entries in row_indices
    pub non_zero_values: Option<Vec<P>>,
    /// Right-hand side, starting guess, initial solutions
    pub rhs_info: Option<Vec<P>>,
}

fn parse_int(buf: &str, field_name: &str) -> i32 {
    buf[..14]
	.trim()
	.parse::<i32>()
	.expect("Failed to parse {field_name}")
}

impl<P: FromStr> HarwellBoeingMatrix<P> {

    pub fn to_vectors(self) -> (Vec<i32>, Vec<i32>, Vec<P>) {
	let non_zero_values = match self.non_zero_values {
	    Some(non_zero_values) => non_zero_values,
	    None => panic!("Missing non-zero values vector in matrix"),
	};
	(self.column_offsets, self.row_indices, non_zero_values)
    }

    pub fn from_file(file: std::fs::File) -> Self {
	let mut reader = io::BufReader::new(file);
	let mut lines = reader.lines();
	
	let line = lines.next()
	    .expect("Expected at least 1 line in Harwell-Boeing file")
	    .expect("Failed to parse line");
	let title = line[0..72].trim().to_string();
	let key = line[72..80].trim().to_string();

	let line = lines.next()
	    .expect("Expected at least 2 line in Harwell-Boeing file")
	    .expect("Failed to parse line");	
	let total_data_lines = parse_int(&line[1*14..],
					 "total_data_lines");
	let num_column_offset_lines = parse_int(&line[1*14..],
						"num_column_offset_lines");
	let num_row_index_lines = parse_int(&line[2*14..],
						"num_row_index_lines");
	let num_values_lines = parse_int(&line[3*14..],
					 "num_values_lines");
	let num_rhs_lines = parse_int(&line[4*14..],
				      "num_rhs_lines");
	
	let line = lines.next()
	    .expect("Expected at least 3 line in Harwell-Boeing file")
	    .expect("Failed to parse line");
	let matrix_type = MatrixType::from_string(line[0..3].trim().to_string());
	let num_rows = parse_int(&line[1*14..],
				 "num_rows");
	let num_columns = parse_int(&line[2*14..],
				    "num_columns");
	let num_non_zeros = parse_int(&line[3*14..],
				       "num_non_zeros");
	let num_elemental_entries = parse_int(&line[4*14..],
				       "num_elemental_entries");
	
	let line = lines.next()
	    .expect("Expected at least 4 line in Harwell-Boeing file")
	    .expect("Failed to parse line");
	let pointer_format = line[0..16].trim().to_string();
	let index_format = line[16..32].trim().to_string();
	let value_format = line[32..52].trim().to_string();
	let rhs_format = line[52..72].trim().to_string();

	let (rhs_type, num_rhs, num_rhs_indices) = if num_rhs_lines > 0 {
	    let line = lines.next()
		.expect("Expected at least 4 line in Harwell-Boeing file")
		.expect("Failed to parse line");
	    let rhs_type = line[0..14].trim().to_string();
	    let num_rhs = parse_int(&line[1*14..],
				    "num_rhs");
	    let num_rhs_indices = parse_int(&line[2*14..],
				    "num_rhs");
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

	println!{"{:?}", header};

	let mut column_offsets = Vec::new();
	for _ in 0..num_column_offset_lines {
	    let int_len = 5;
	    let line = lines.next()
		.expect("Failed to read line while parsing column pointers")
		.unwrap();
	    assert!(line.len() % int_len == 0,
		    "Integer (offset) line length not a multiple of {int_len}");
	    for k in 0..line.len()/int_len {
		column_offsets.push(line[int_len*k..][..int_len]
				    .trim()
				    .parse::<i32>()
				    .expect("Failed to parse offset as integer"));
	    }	    
	}
	
	let mut row_indices = Vec::new();
	for _ in 0..num_row_index_lines {
	    let int_len = 5 ;
	    let line = lines.next()
		.expect("Failed to read line while parsing row indices")
		.unwrap();
	    assert!(line.len() % int_len == 0,
		    "Integer (offset) line length not a multiple of {int_len}");
	    for k in 0..line.len()/int_len {
		row_indices.push(line[int_len*k..][..int_len]
				 .trim()
				 .parse::<i32>()
				 .expect("Failed to parse index as integer"));
	    }	    
	}

	let mut non_zero_values = Vec::new();
	for _ in 0..num_values_lines {
	    let fp_len = 15;
	    let line = lines.next()
		.expect("Failed to read line while parsing row indices")
		.unwrap();
	    println!("{line}");
	    assert!(line.len() % fp_len == 0,
		    "Values line length not a multiple of {fp_len}");
	    for k in 0..line.len()/fp_len {
		let value = match line[fp_len*k..][..fp_len]
		    .trim()
		    .parse::<P>() {
			Ok(value) => value,
			Err(_) => panic!("Failed"),
		    };
		non_zero_values.push(value)
	    }	    
	}

	Self {
	    header,
	    column_offsets,
	    row_indices,
	    non_zero_values: Some(non_zero_values),
	    rhs_info: None,
	}
    }
}
