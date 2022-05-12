use serde::Serialize;

#[derive(Debug, FromForm, Default, Serialize)]
pub struct ErrorContext {
    pub info: String,
}
