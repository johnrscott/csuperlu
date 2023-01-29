//! This example is the same as the one in section 2.2 of the SuperLU manual.
//!
//! From the original source code:
//! " This is the small 5x5 example used in the Sections 2 and 3 of the
//!   Users’ Guide to illustrate how to call a SuperLU routine, and the
//!   matrix data structures used by SuperLU. "
//!
//! This code calls all the same functions as the C code, but with names
//! prefixed with c_. However, there are a few differences compared to the
//! C code. First, Rust vectors are used instead of C style vectors (allocated
//! with malloc). Second, Rust requires unsafe blocks whenever a struct is
//! initialised by a function taking the uninitialised struct as an input/output
//! parameter.

use csuperlu::c::comp_col::c_Destroy_CompCol_Matrix;
use csuperlu::c::options::{colperm_t, superlu_options_t};
use csuperlu::c::stat::{c_StatPrint, SuperLUStat_t};
use csuperlu::c::super_node::{c_Destroy_SuperNode_Matrix, c_dPrint_SuperNode_Matrix};
use csuperlu::c::super_matrix::{Dtype_t, Mtype_t, Stype_t};
use csuperlu::comp_col::CompColMatrix;
use csuperlu::dense::DenseMatrix;
use csuperlu::simple_driver::{dgssv, DgssvSolution};
use csuperlu::super_matrix::SuperMatrix;

fn main() {
    // Matrix dimensions
    let m: i32 = 5;
    let n: i32 = 5;

    // Number of non-zeros
    let nnz: i32 = 12;

    // Matrix elements
    let s: f64 = 19.0;
    let u: f64 = 21.0;
    let p: f64 = 16.0;
    let e: f64 = 5.0;
    let r: f64 = 18.0;
    let l: f64 = 12.0;

    // Vector of doubles of length nnz
    let a = vec![s, l, l, u, l, l, u, p, u, e, u, r];

    // Vector of ints of length nnz
    let asub = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

    // Vector of ints of length n+1
    let xa = vec![0, 3, 6, 8, 10, 12];

    // Make the left-hand side matrix
    let mut a = CompColMatrix::new(
        m,
        n,
        nnz,
        a,
        asub,
        xa,
        Mtype_t::SLU_GE,
    );

    // Make the RHS vector
    let nrhs = 1;
    let rhs = vec![1.0; m as usize];
    let b = DenseMatrix::new(
        m,
        nrhs,
        rhs,
        m,
        Mtype_t::SLU_GE,
    );

    let mut options = superlu_options_t::new();
    options.ColPerm = colperm_t::NATURAL;

    let mut perm_r = Vec::<i32>::with_capacity(m as usize);
    let mut perm_c = Vec::<i32>::with_capacity(n as usize);

    let stat = SuperLUStat_t::new();

    let DgssvSolution {
        mut X,
        mut L,
        mut U,
        mut stat,
        mut info,
    } = dgssv(options, &mut a, &mut perm_c, &mut perm_r, b, stat);

    // Print the performance statistics
    c_StatPrint(&mut stat);

    //dPrint_CompCol_Matrix("A", &mut A.super_matrix());
    a.print("a");
    //U.print("U");

    let c_str = std::ffi::CString::new("L").unwrap();
    c_dPrint_SuperNode_Matrix(c_str.as_ptr() as *mut libc::c_char, &mut L);

    c_Destroy_SuperNode_Matrix(&mut L);
    c_Destroy_CompCol_Matrix(&mut U);
}
