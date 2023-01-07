mod c_utils;

use std::mem::MaybeUninit;
use crate::superlu::utils::c_utils::{
    c_set_default_options,
    c_StatInit,
    c_StatFree,
};

// Data type
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Dtype_t {
    SLU_S,
    SLU_D,
    SLU_C,
    SLU_Z
}

// Specifies some mathematical properties
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Mtype_t {
    SLU_GE,
    SLU_TRLU,
    SLU_TRUU,
    SLU_TRL,
    SLU_TRU,
    SLU_SYL,
    SLU_SYU,
    SLU_HEL,
    SLU_HEU,
}

// Storage type
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Stype_t {
    SLU_NC,
    SLU_NCP,
    SLU_NR,
    SLU_SC,
    SLU_SCP,
    SLU_SR,
    SLU_DN,
    SLU_NR_loc,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum yes_no_t {
    NO,
    YES,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fact_t {
    DOFACT,
    SamePattern,
    SamePattern_SameRowPerm,
    FACTORED,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum colperm_t {
    NATURAL,
    MMD_ATA,
    MMD_AT_PLUS_A,
    COLAMD,
    METIS_AT_PLUS_A,
    PARAMETIS,
    ZOLTAN,
    MY_PERMC,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum trans_t {
    NOTRANS,
    TRANS,
    CONJ,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum IterRefine_t {
    NOREFINE,
    SLU_SINGLE,
    SLU_DOUBLE,
    SLU_EXTRA,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum rowperm_t {
    NOROWPERM,
    LargeDiag_MC64,
    LargeDiag_HWPM,
    MY_PERMR,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum norm_t {
    ONE_NORM,
    TWO_NORM,
    INF_NORM,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum milu_t {
    SILU,
    SMILU_1,
    SMILU_2,
    SMILU_3,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct superlu_options_t {
    Fact: fact_t,
    Equil: yes_no_t,
    pub ColPerm: colperm_t,
    Trans: trans_t,
    IterRefine: IterRefine_t,
    DiagPivotThresh: libc::c_double,
    SymmetricMode: yes_no_t,
    PivotGrowth: yes_no_t,
    ConditionNumber: yes_no_t,
    RowPerm: rowperm_t,
    ILU_DropRule: libc::c_int,
    ILU_DropTol: libc::c_double,
    ILU_FillFactor: libc::c_double,
    ILU_Norm: norm_t,
    ILU_FillTol: libc::c_double,
    ILU_MILU: milu_t,
    ILU_MILU_Dim: libc::c_double,
    ParSymbFact: yes_no_t,
    ReplaceTinyPivot: yes_no_t,
    SolveInitialized: yes_no_t,
    RefineInitialized: yes_no_t,
    PrintStat: yes_no_t,
    nnzL: libc::c_int,
    nnzU: libc::c_int,
    num_lookaheads: libc::c_int,
    lookahead_etree: yes_no_t,
    SymPattern: yes_no_t,
}

impl superlu_options_t {
    pub fn new() -> Self {
	let options = unsafe {
	    let mut options = MaybeUninit::<Self>::uninit();
	    set_default_options(options.as_mut_ptr());
	    options.assume_init()
	};
	options
    }
}

#[allow(non_camel_case_types)]
pub type flops_t = libc::c_float;

#[repr(C)]
#[allow(non_snake_case)]
pub struct SuperLUStat_t {
    panel_histo: *mut libc::c_int,
    utime: *mut libc::c_double,
    ops: *mut flops_t,
    TinyPivots: libc::c_int,
    RefineSteps: libc::c_int,
    expansions: libc::c_int,
}

impl SuperLUStat_t {
    pub fn new() -> Self {
	let stat = unsafe {
    	    let mut stat = MaybeUninit::<SuperLUStat_t>::uninit();
    	    StatInit(stat.as_mut_ptr());
    	    stat.assume_init()
	};
	stat
    }
}

impl Drop for SuperLUStat_t {
    fn drop(&mut self) {
	unsafe {
	    StatFree(self);	    
	}
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct SuperMatrix {
    Stype: Stype_t,
    Dtype: Dtype_t,
    Mtype: Mtype_t,
    nrow: libc::c_int,
    ncol: libc::c_int,
    Store: *mut libc::c_void,
}

impl SuperMatrix {
    pub fn num_rows(&self) -> usize {
	self.nrow as usize
    }
    pub fn num_cols(&self) -> usize {
	self.ncol as usize
    }
}

pub fn set_default_options(options: *mut superlu_options_t) {
    c_set_default_options(options);
}

pub fn StatInit(stat: *mut SuperLUStat_t) {
    c_StatInit(stat);
}


pub fn StatFree(stat: *mut SuperLUStat_t) {
    c_StatFree(stat);
}
