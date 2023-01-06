//! Rust interface to SuperLU, a C library for solving sparse linear systems.
//! The crate is designed to provide a wrapper around SuperLU that is safe,
//! performative, and retains the main features of the original SuperLU iterface.
//!
//! The SuperLU User Guide is
//! <https://portal.nersc.gov/project/sparse/superlu/superlu_ug.pdf>
//!
//! The function reference for SuperLU is provided  
//! <https://portal.nersc.gov/project/sparse/superlu/superlu_code_html/index.html>

#![warn(missing_docs)]
mod superlu;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
