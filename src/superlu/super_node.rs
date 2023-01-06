use libc;
use crate::superlu::utils::SuperMatrix;

#[link(name = "superlu")]
extern {
    fn dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix);
    fn Destroy_SuperNode_Matrix(A: *mut SuperMatrix);    
}

pub struct SuperNodeMatrix {
    pub super_matrix: SuperMatrix,
}

impl SuperNodeMatrix {
    pub fn from_super_matrix(super_matrix: SuperMatrix) -> Self {
    	Self {
    	    super_matrix
    	}
    }
    pub fn print(&mut self, label: &str) {
	let c = std::ffi::CString::new(label).unwrap();
	unsafe {
	    dPrint_SuperNode_Matrix(c.as_ptr() as *mut libc::c_char,
				    &mut self.super_matrix);
	}
    }
}

impl Drop for SuperNodeMatrix {
    fn drop(&mut self) {
	unsafe {
	    Destroy_SuperNode_Matrix(&mut self.super_matrix);
	}
    }
}
