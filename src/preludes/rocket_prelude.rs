pub use crate::models::*;
pub use rocket::form::{Context, Contextual, Form, FromForm, FromFormField};
pub use rocket::fs::{relative, FileServer, TempFile};
pub use rocket::http::{ContentType, Status};
pub use rocket::serde::Serialize;
pub use rocket_db_pools::sqlx::{self, Row};
pub use rocket_db_pools::{Connection, Database};
pub use rocket_dyn_templates::Template;

#[derive(Database)]
#[database("bank_manage")]
pub struct BankManage(sqlx::MySqlPool);
