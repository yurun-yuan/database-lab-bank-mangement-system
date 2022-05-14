use std::{ops::Mul, usize};

use serde::Serialize;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, FromForm, Default, Serialize)]
pub struct ErrorContext {
    pub info: String,
}

#[macro_export]
macro_rules! start_transaction {
    ($db: ident) => {
        match $db.execute("START TRANSACTION").await {
            Ok(_) => (),
            Err(e) => panic!("Error starting a transaction: {}", e.to_string()),
        }
    };
}

#[macro_export]
macro_rules! rollback {
    ($db:ident) => {
        match $db.execute("ROLLBACK").await {
            Ok(_) => (),
            Err(e) => panic!("Error rolling back: {}", e.to_string()),
        }
    };
}

#[macro_export]
macro_rules! commit {
    ($db:ident) => {
        match $db.execute("COMMIT").await {
            Ok(_) => (),
            Err(e) => panic!("Error committing: {}", e.to_string()),
        }
    };
}

pub fn f64_to_decimal(float: f64, precision: i32) -> String {
    float.mul(10f64.powi(precision)).round().to_string()
}
