//! Contains a trait for supported numerical value types in the
//! C SuperLU library. The supported values types are float (f32),
//! double (f64), complex float (num::Complex<f32>), and complex
//! double (num::Complex<f64>). 

use std::{str::FromStr, mem::MaybeUninit};

use num::Num;

use csuperlu_sys::{super_matrix::{c_SuperMatrix, Mtype_t, Stype_t, Dtype_t}, options::superlu_options_t, stat::SuperLUStat_t, comp_col::{sCreate_CompCol_Matrix, sPrint_CompCol_Matrix, dCreate_CompCol_Matrix, dPrint_CompCol_Matrix, cCreate_CompCol_Matrix, cPrint_CompCol_Matrix, zCreate_CompCol_Matrix, zPrint_CompCol_Matrix}, dense::{sCreate_Dense_Matrix, sPrint_Dense_Matrix, dCreate_Dense_Matrix, dPrint_Dense_Matrix, cCreate_Dense_Matrix, cPrint_Dense_Matrix, zCreate_Dense_Matrix, zPrint_Dense_Matrix}, super_node::{sPrint_SuperNode_Matrix, dPrint_SuperNode_Matrix, cPrint_SuperNode_Matrix, zPrint_SuperNode_Matrix}, simple_driver::{sgssv, dgssv, cgssv, zgssv}};

/// Valid numerical value types for the C SuperLU library
///
pub trait ValueType<P>: Num + Copy + FromStr + std::fmt::Debug {

    /// Create a compressed-column matrix from raw vectors
    ///
    fn c_create_comp_col_matrix(
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<P>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> c_SuperMatrix;

    /// Print a compressed-column matrix (from SuperLU library)
    ///
    /// The function makes the assumption that the C library does
    /// not modify the arguments.
    fn c_print_comp_col_matrix(what: *const libc::c_char, a: &c_SuperMatrix);
    fn c_create_dense_matrix(
        m: i32,
        n: i32,
        values: &mut Vec<P>,
        ldx: i32,
        mtype: Mtype_t,
    ) -> c_SuperMatrix;
    fn c_print_dense_matrix(what: *const libc::c_char, a: &c_SuperMatrix);
    fn c_print_super_node_matrix(what: *const libc::c_char, a: &c_SuperMatrix);
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    );
}

impl ValueType<f32> for f32 {
    fn c_create_comp_col_matrix(
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<f32>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> c_SuperMatrix {
	let mut a = c_SuperMatrix::alloc();
        unsafe {
            sCreate_CompCol_Matrix(
                &a as *mut c_SuperMatrix,
                m,
                n,
                nnz,
                nzval.as_mut_ptr(),
                rowind.as_mut_ptr(),
                colptr.as_mut_ptr(),
                Stype_t::SLU_NC,
                Dtype_t::SLU_S,
                mtype,
            );
	    a
        }
    }

    fn c_print_comp_col_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
            sPrint_CompCol_Matrix(what as *mut libc::c_char,
				  a as *const c_SuperMatrix as *mut c_SuperMatrix);
        }
    }
    
    fn c_create_dense_matrix(
        m: i32,
        n: i32,
        values: &mut Vec<f32>,
        ldx: i32,
        mtype: Mtype_t,
    ) -> c_SuperMatrix {
	let mut x = c_SuperMatrix::alloc();
        unsafe {
            sCreate_Dense_Matrix(
                &x as *mut c_SuperMatrix,
                m,
                n,
                values.as_mut_ptr(),
                ldx,
                Stype_t::SLU_DN,
                Dtype_t::SLU_S,
                mtype,
            );
	    x
	}
    }

    fn c_print_dense_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    sPrint_Dense_Matrix(what as *mut libc::c_char,
				a as *const c_SuperMatrix as *mut c_SuperMatrix);
        }
    }
    
    fn c_print_super_node_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
            sPrint_SuperNode_Matrix(what as *mut libc::c_char,
				    a as *const c_SuperMatrix as *mut c_SuperMatrix);
	    
	}
    }
    
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
	unsafe {
            sgssv(options, a, perm_c.as_mut_ptr(), perm_r.as_mut_ptr(),
		  l, u, b, stat, info);
	}	
    }

}

impl ValueType<f64> for f64 {
    fn c_create_comp_col_matrix(
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<f64>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> c_SuperMatrix {
	let mut a = c_SuperMatrix::alloc();
        unsafe {
            dCreate_CompCol_Matrix(
                &a as *mut c_SuperMatrix,
                m,
                n,
                nnz,
                nzval.as_mut_ptr(),
                rowind.as_mut_ptr(),
                colptr.as_mut_ptr(),
                Stype_t::SLU_NC,
                Dtype_t::SLU_D,
                mtype,
            );
	    a
	}
    }

    fn c_print_comp_col_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
            dPrint_CompCol_Matrix(what as *mut libc::c_char,
				  a as *const c_SuperMatrix as *mut c_SuperMatrix);

	}
    }
    fn c_create_dense_matrix(
        m: i32,
        n: i32,
        values: &mut Vec<f64>,
        ldx: i32,
        mtype: Mtype_t,
    ) -> c_SuperMatrix {
        unsafe {
	    let mut x = c_SuperMatrix::alloc();
            dCreate_Dense_Matrix(
                &x as *mut c_SuperMatrix,
                m,
                n,
                values.as_mut_ptr(),
                ldx,
                Stype_t::SLU_DN,
                Dtype_t::SLU_D,
                mtype,
            );
	    x
	}
    }

    fn c_print_dense_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    dPrint_Dense_Matrix(what as *mut libc::c_char,
				a as *const c_SuperMatrix as *mut c_SuperMatrix);
        }
    }
    fn c_print_super_node_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    dPrint_SuperNode_Matrix(what as *mut libc::c_char,
				    a as *const c_SuperMatrix as *mut c_SuperMatrix);
        }
    }
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
	unsafe {
            dgssv(options, a, perm_c.as_mut_ptr(), perm_r.as_mut_ptr(),
		  l, u, b, stat, info);
	}	
    }

}

impl ValueType<num::Complex<f32>> for num::Complex<f32> {
    fn c_create_comp_col_matrix(
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<num::Complex<f32>>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> c_SuperMatrix {
	let mut a = c_SuperMatrix::alloc();
        unsafe {
            cCreate_CompCol_Matrix(
                &a as *mut c_SuperMatrix,
                m,
                n,
                nnz,
                nzval.as_mut_ptr() as *mut libc::c_float,
                rowind.as_mut_ptr(),
                colptr.as_mut_ptr(),
                Stype_t::SLU_NC,
                Dtype_t::SLU_C,
                mtype,
            );
	    a
        }
    }

    fn c_print_comp_col_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    cPrint_CompCol_Matrix(what as *mut libc::c_char,
				  a as *const c_SuperMatrix as *mut c_SuperMatrix);
        }
    }
    fn c_create_dense_matrix(
        m: i32,
        n: i32,
        values: &mut Vec<num::Complex<f32>>,
        ldx: i32,
        mtype: Mtype_t,
    ) -> c_SuperMatrix {
	let mut x = c_SuperMatrix::alloc();
        unsafe {
            cCreate_Dense_Matrix(
                &x as *mut c_SuperMatrix,
                m,
                n,
                values.as_mut_ptr() as *mut libc::c_float,
                ldx,
                Stype_t::SLU_DN,
                Dtype_t::SLU_C,
                mtype,
            );
        }
    }

    fn c_print_dense_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    cPrint_Dense_Matrix(what as *mut libc::c_char,
				a as *const c_SuperMatrix as *mut c_SuperMatrix);
        }
    }
    fn c_print_super_node_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    cPrint_SuperNode_Matrix(what as *mut libc::c_char,
				    a as *const c_SuperMatrix as *mut c_SuperMatrix);
        }
    }
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
	unsafe {
            cgssv(options, a, perm_c.as_mut_ptr(), perm_r.as_mut_ptr(),
		  l, u, b, stat, info);
	}	
    }
}

impl ValueType<num::Complex<f64>> for num::Complex<f64> {
    fn c_create_comp_col_matrix(
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<num::Complex<f64>>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> c_SuperMatrix {
	let mut a = c_SuperMatrix::alloc();
        unsafe {
            zCreate_CompCol_Matrix(
                a.as_mut_ptr(),
                m,
                n,
                nnz,
                nzval.as_mut_ptr() as *mut libc::c_double,
                rowind.as_mut_ptr(),
                colptr.as_mut_ptr(),
                Stype_t::SLU_NC,
                Dtype_t::SLU_Z,
                mtype,
            );
        }
	a
    }

    fn c_print_comp_col_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    zPrint_CompCol_Matrix(what as *mut libc::c_char,
				  a as *const c_SuperMatrix as *mut c_SuperMatrix);
	}
    }

    fn c_create_dense_matrix(
        m: i32,
        n: i32,
        values: &mut Vec<num::Complex<f64>>,
        ldx: i32,
        mtype: Mtype_t,
    ) -> c_SuperMatrix {
        unsafe {
	    let mut x = c_SuperMatrix::alloc();
            zCreate_Dense_Matrix(
                &x as *mut c_SuperMatrix,
                m,
                n,
                values.as_mut_ptr() as *mut libc::c_double,
                ldx,
                Stype_t::SLU_DN,
                Dtype_t::SLU_Z,
                mtype,
            );
	    x
        }
    }

    fn c_print_dense_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    zPrint_Dense_Matrix(what as *mut libc::c_char,
				a as *const c_SuperMatrix as *mut c_SuperMatrix);
	}
    }

    fn c_print_super_node_matrix(what: *const libc::c_char, a: &c_SuperMatrix) {
        unsafe {
	    zPrint_SuperNode_Matrix(what as *mut libc::c_char,
				    a as *const c_SuperMatrix as *mut c_SuperMatrix);
	    
        }
    }

    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
	unsafe {
            zgssv(options, a, perm_c.as_mut_ptr(), perm_r.as_mut_ptr(),
		  l, u, b, stat, info);
	}	
    }
}
