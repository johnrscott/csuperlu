use std::{io::{self, BufRead}, str::FromStr};

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
    matrix_type: String,
    /// Number of rows in the matrix
    num_rows: i32,
    /// Number of columns in the matrix
    num_columns: i32,
    /// Number of non-zero values in the matrix
    num_non_zeroes: i32,
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
pub struct HarwellBoeingMatrix<P> {
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

fn parse_int(buf: &str, field_name: &str) -> i32 {
    buf[..14]
	.trim()
	.parse::<i32>()
	.expect("Failed to parse {field_name}")
}

impl<P> HarwellBoeingMatrix<P> {
    pub fn from_file(file: std::fs::File) -> HarwellBoeingHeader {
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
	let num_row_index_lines = parse_int(&line[1*14..],
						"num_row_index_lines");
	let num_values_lines = parse_int(&line[3*14..],
					 "num_values_lines");
	let num_rhs_lines = parse_int(&line[4*14..],
				      "num_rhs_lines");
	
	let line = lines.next()
	    .expect("Expected at least 3 line in Harwell-Boeing file")
	    .expect("Failed to parse line");
	let matrix_type = line[0..3].trim().to_string();
	let num_rows = parse_int(&line[1*14..],
				 "num_rows");
	let num_columns = parse_int(&line[2*14..],
				    "num_columns");
	let num_non_zeroes = parse_int(&line[3*14..],
				       "num_non_zeroes");
	let num_elemental_entries = parse_int(&line[4*14..],
				       "num_elemental_entries");
	
	let line = lines.next()
	    .expect("Expected at least 4 line in Harwell-Boeing file")
	    .expect("Failed to parse line");
	let pointer_format = line[0..14].trim().to_string();
	let index_format = line[14..28].trim().to_string();
	let value_format = line[28..42].trim().to_string();
	let rhs_format = line[42..56].trim().to_string();

	let (rhs_type, num_rhs, num_rhs_indices) = if num_rhs_lines > 0 {
	    let line = lines.next()
		.expect("Expected at least 4 line in Harwell-Boeing file")
		.expect("Failed to parse line");
	    let rhs_type = line[0..14].trim().to_string();
	    let num_rhs = parse_int(&line[1*14..],
				    "num_rhs");
	    let num_rhs_indices = parse_int(&line[2*14..],
				    "num_rhs");
	    let index_format = &line[14..28].trim();
	    (Some(rhs_type), Some(num_rhs), Some(num_rhs_indices))
	} else {
	    (None, None, None)
	};

	HarwellBoeingHeader {  
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
	    num_non_zeroes,
	    num_elemental_entries,
	    pointer_format,
	    index_format,
	    value_format,
	    rhs_format,
	    rhs_type,
	    num_rhs,
	    num_rhs_indices,
	}
	
	// for line in lines {
	//     println!("{}", line.unwrap());
	// }
    }
}
