//! Contains a trait with SuperLU functions that depend on precision
//!
//! Contains a trait for supported numerical value types in the
//! C SuperLU library. The supported values types are float (f32),
//! double (f64), complex float (num::Complex<f32>), and complex
//! double (num::Complex<f64>).
//!

use std::{fmt, str, ops::AddAssign};

use num::{Num, Float, traits::real::Real, FromPrimitive};

use super::simple_driver::SimpleDriver;

/// Valid numerical value types for the C SuperLU library
///
pub trait ValueType: Num + Copy + str::FromStr + fmt::Debug + SimpleDriver {
    type RealType: Real + AddAssign + FromPrimitive;
    fn abs(self) -> Self::RealType;    
}

impl ValueType for f32 {
    type RealType = f32;
    fn abs(self) -> Self::RealType {
	return Self::RealType::abs(self)
    }
}

impl ValueType for f64 {
    type RealType = f64;
    fn abs(self) -> Self::RealType {
	return Self::RealType::abs(self)
    }
}

impl ValueType for num::Complex<f32> {
    type RealType = f32;
    fn abs(self) -> Self::RealType {
	return self.norm()
    }
}

impl ValueType for num::Complex<f64> {
    type RealType = f64;
    fn abs(self) -> Self::RealType {
	return self.norm()
    }
}
