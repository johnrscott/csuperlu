use std::mem::MaybeUninit;

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
#[allow(non_camel_case_types)]
pub struct superlu_options_t {
    pub Fact: fact_t,
    pub Equil: yes_no_t,
    pub ColPerm: colperm_t,
    pub Trans: trans_t,
    pub IterRefine: IterRefine_t,
    pub DiagPivotThresh: libc::c_double,
    pub SymmetricMode: yes_no_t,
    pub PivotGrowth: yes_no_t,
    pub ConditionNumber: yes_no_t,
    pub RowPerm: rowperm_t,
    pub ILU_DropRule: libc::c_int,
    pub ILU_DropTol: libc::c_double,
    pub ILU_FillFactor: libc::c_double,
    pub ILU_Norm: norm_t,
    pub ILU_FillTol: libc::c_double,
    pub ILU_MILU: milu_t,
    pub ILU_MILU_Dim: libc::c_double,
    pub ParSymbFact: yes_no_t,
    pub ReplaceTinyPivot: yes_no_t,
    pub SolveInitialized: yes_no_t,
    pub RefineInitialized: yes_no_t,
    pub PrintStat: yes_no_t,
    pub nnzL: libc::c_int,
    pub nnzU: libc::c_int,
    pub num_lookaheads: libc::c_int,
    pub lookahead_etree: yes_no_t,
    pub SymPattern: yes_no_t,
}

#[link(name = "superlu")]
extern {
    fn set_default_options(options: *mut superlu_options_t);
}

pub fn c_set_default_options(options: *mut superlu_options_t) {
    unsafe {
	set_default_options(options);
    }
}

impl superlu_options_t {
    pub fn new() -> Self {
	unsafe {
	    let mut options = MaybeUninit::<superlu_options_t>::uninit();
	    c_set_default_options(options.as_mut_ptr());
	    options.assume_init()
	}
    }
}
