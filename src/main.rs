#![allow(non_snake_case)]
#![feature(generic_arg_infer)]

#[macro_use]
pub extern crate rocket;

extern crate dotenv;

pub mod models;
mod new_client;
mod preludes;
mod search;
use preludes::rocket_prelude::*;
mod account_manage;
mod account_profile;
mod client_profile;
mod delete_payment;
mod edit_account;
mod edit_client;
mod loan_profile;
mod new_account;
mod new_loan;
mod new_payment;
mod utility;

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
                new_account::submit,
                account_profile::account_profile,
                edit_account::get_edit_account,
                edit_account::act_edit_saving_account,
                edit_account::act_edit_checking_account,
                edit_account::delete_account,
                new_loan::get_new_loan,
                new_loan::submit,
                loan_profile::loan_profile,
                new_payment::get_new_loan,
                new_payment::submit,
                delete_payment::delete_payment
            ],
        )
        .attach(Template::fairing())
        .attach(BankManage::init())
        .mount("/", FileServer::from(relative!("/static")))
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
    (Account)=>{
        get_attr_list!(Account; accountID)
    };
    {Loan}=>{
        get_attr_list!(Loan; loanID)
    }
}

#[get("/")]
fn index() -> Template {
    let mut options = vec![];
    options.extend(get_attr_list_of!(Client).into_iter());
    options.extend(get_attr_list_of!(Account).into_iter());
    options.extend(get_attr_list_of!(Loan).into_iter());
    Template::render("index", &IndexContext { options })
}
