
/// Data contained in the header of a Harwell-Boeing
/// matrix file.
///
/// The Harwell-Boeing format is described
/// [here](https://people.sc.fsu.edu/~jburkardt/data/hb/hb.html).
///
struct HarwellBoeingHeader {
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
    nrow: i32,
    /// Number of columns in the matrix
    ncolumns: i32,
    /// Number of non-zero values in the matrix
    num_non_zeroes: i32,
    num_elemental_entries: i32,
    pointer_format: String,
    index_format: String,
    value_format: String,
    rhs_format: String,
    rhs_type: String,
    /// Number of right-hand sides
    num_rhs: i32,
    num_row_indices: i32,
}

/// Sparse matrix stored in Harwell-Boeing format. Often, the
/// matrix may not contain right-hand side information (in this
/// case it stores a single sparse matrix in compressed-column
/// format). Sometimes, the non-zero values may be omitted, in
/// which case the matrix is called a "pattern matrix".
/// 
struct HarwellBoeingMatrix<P> {
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

