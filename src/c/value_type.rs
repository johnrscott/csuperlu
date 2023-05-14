//! Contains a trait with SuperLU functions that depend on precision
//!
//! Contains a trait for supported numerical value types in the
//! C SuperLU library. The supported values types are float (f32),
//! double (f64), complex float (num::Complex<f32>), and complex
//! double (num::Complex<f64>).
//!

use std::{fmt, str};

use num::Num;

use super::simple_driver::SimpleDriver;

/// Valid numerical value types for the C SuperLU library
///
pub trait ValueType: Num + Copy + str::FromStr + fmt::Debug + SimpleDriver {}

impl ValueType for f32 {}
impl ValueType for f64 {}
impl ValueType for num::Complex<f32> {}
impl ValueType for num::Complex<f64> {}
