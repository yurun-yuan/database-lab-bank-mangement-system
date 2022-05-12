#![allow(non_snake_case)]
#![feature(generic_arg_infer)]

#[macro_use]
pub extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
mod new_client;
mod preludes;
pub mod schema;
mod search;
use preludes::diesel_prelude::*;
use preludes::rocket_prelude::*;
mod client_profile;
mod edit_client;
mod new_account;
mod utility;

use rocket_sync_db_pools::database;

#[database("bank_manage")]
pub struct BMDBConn(diesel::MysqlConnection);

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                search::search,
                new_client::submit,
                new_client::new_client,
                client_profile::client_profile,
                edit_client::get_edit_client,
                edit_client::act_edit_client,
                edit_client::delete_client,
                new_account::new_account,
                new_account::submit
            ],
        )
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
        .attach(BMDBConn::fairing())
}

#[derive(Serialize)]
struct IndexContext {
    options: Vec<String>,
}

#[macro_export]
macro_rules! get_attr_list_of {
    (Client) => {
        get_attr_list!(Client; clientID,clientName,clientAddr,contactName)
    };
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        &IndexContext {
            options: get_attr_list_of!(Client),
        },
    )
}
