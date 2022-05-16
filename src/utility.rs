use std::{collections::HashMap, usize};

use serde::Serialize;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, FromForm, Default, Serialize)]
pub struct ErrorContext {
    pub info: String,
}

pub fn get_list_from_input<Container: std::iter::FromIterator<std::string::String>>(
    input: &str,
) -> Container {
    input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Container>()
}

// If `value` is not empty, wraps it with "'"; else returns "NULL"
pub fn validate_string_value(value: &str) -> String {
    if value.is_empty() {
        "NULL".to_string()
    } else {
        format!("'{value}'").to_string()
    }
}

macro_rules! str_map {
    ($($key: expr, $value: expr);+) => {
        HashMap::<String, String>::from([
            $(($key.to_string(), $value.to_string()),)+
        ])
    };
    ($($key: expr, $value: expr);+;)=>{str_map!($($key, $value);+)};
}

pub type Restriction = HashMap<String, String>;

pub fn get_restriction() -> Restriction {
    let clientID = r"[0-9]{17}([0-9]|X|x)";
    str_map!(
        "name",
        r"[\u4e00-\u9fa5a-zA-Z]([\u4e00-\u9fa5a-zA-Z ]*[\u4e00-\u9fa5a-zA-Z])?"; // chinese/english characters, spaces are permitted in the middle
        "clientID", clientID;
        "tel", r"\+?[0-9]*";
        "email", r"^[a-zA-Z0-9]{1,10}@[a-zA-Z0-9]{1,5}\.[a-zA-Z0-9]{1,5}$";
        "id_list", format!(r"\s*{clientID}(\s+{clientID})*\s*");
        "amount", r"[0-9]{1,62}(\.[0-9]{1,2})?";
        "currency_type", r"[a-zA-Z]+";
        "float", r"[0-9]{1,}(\.[0-9]{1,})?"
    )
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
    ($error: ident, $info: expr) => {
        Template::render(
            "error",
            &$crate::utility::ErrorContext {
                info: format!("{}: {}", $info, $error.to_string()),
            },
        )
    };
    ($error: ident) => {
        error_template!($error, "Exception occurs! ")
    };
    ($info: expr) => {
        Template::render(
            "error",
            &$crate::utility::ErrorContext {
                info: format!("{}", $info),
            },
        )
    };
}

#[macro_export]
macro_rules! unwrap_or {
    ($result: expr,$error: ident, $or: block) => {
        match $result {
            Ok(o) => o,
            Err($error) => $or,
        }
    };
}
#[macro_export]
macro_rules! unwrap_or_return {
    ($result:  expr, $info: literal) => {
        $crate::unwrap_or!($result, e, { return crate::error_template!(e, $info) })
    };
}
