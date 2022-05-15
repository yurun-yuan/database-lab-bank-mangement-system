use std::collections::HashMap;

use super::preludes::rocket_prelude::*;
use crate::{account_manage::insert::*, start_transaction};
use crate::{commit, error_template, rollback};
use sqlx::Executor;

#[get("/new/account")]
pub fn new_account() -> Template {
    Template::render("new-account", &Context::default())
}

#[post("/new/account", data = "<form>")]
pub async fn submit(
    mut db: Connection<BankManage>,
    form: Form<Contextual<'_, AccountSubmit>>,
) -> (Status, Template) {
    let template;
    match form.value {
        Some(ref submission) => {
            start_transaction!(db);
            let result = add_new_account_and_own(&mut db, submission).await;
            match result {
                Ok(id) => {
                    commit!(db);
                    template = Template::render("new-account-success", &HashMap::from([("id", &id)]))
                }
                Err(e) => {
                    rollback!(db);
                    template = error_template!(e, "Error inserting account");
                }
            }
        }
        None => template = error_template!("Error inserting new account: failed to receive form"),
    };

    (form.context.status(), template)
}
