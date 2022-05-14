use std::{ops::Mul, usize};

use serde::Serialize;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, FromForm, Default, Serialize)]
pub struct ErrorContext {
    pub info: String,
}

pub fn f64_to_decimal(float: f64, precision: i32) -> String {
    float.mul(10f64.powi(precision)).round().to_string()
}
