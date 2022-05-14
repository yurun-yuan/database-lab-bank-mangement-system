use super::preludes::rocket_prelude::*;
use crate::account_manage::insert::*;
use crate::utility::ErrorContext;
use chrono::prelude::*;
use rocket::futures::TryStreamExt;
use sqlx::{query_as, Executor};

#[derive(Debug, FromForm, Default, Serialize)]
pub struct ClientBasicInfoContext {
    id: String,
    name: String,
}

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
            db.execute("START TRANSACTION")
                .await
                .expect("Error starting a transaction");
            let result = add_new_account_and_own(&mut db, submission).await;
            match result {
                Ok(()) => template = Template::render("new-account-success", &form.context),
                Err(e) => {
                    db.execute("ROLLBACK").await.expect(&format!(
                        "Error rolling back: {e_info}",
                        e_info = e.to_string()
                    ));
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
            db.execute("COMMIT").await.expect("Error committing");
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
