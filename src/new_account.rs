use super::preludes::rocket_prelude::*;
use crate::utility::ErrorContext;
use crate::{account_manage::insert::*, start_transaction};
use crate::{commit, rollback};
use chrono::prelude::*;
use rocket::futures::TryStreamExt;
use sqlx::{query_as, Executor};

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
                Ok(()) => {
                    commit!(db);
                    template = Template::render("new-account-success", &form.context)
                }
                Err(e) => {
                    rollback!(db);
                    template = Template::render(
                        "error",
                        &ErrorContext {
                            info: format!(
                                "Error inserting account: {e_info}",
                                e_info = e.to_string()
                            ),
                        },
                    );
                }
            }
        }
        None => {
            template = Template::render(
                "error",
                &ErrorContext {
                    info: format!("Error inserting new account: failed to receive form"),
                },
            );
        }
    };

    (form.context.status(), template)
}
