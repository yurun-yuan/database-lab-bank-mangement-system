use std::usize;

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

#[macro_export]
macro_rules! error_template {
    ($error: ident, $info: literal) => {
        Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("{}: {}", $info, $error.to_string()),
            },
        )
    };
    ($error: ident) => {
        error_template!($error, "Exception occurs! ")
    };
    ($info: literal) => {
        Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("{}", $info),
            },
        )
    };
}
