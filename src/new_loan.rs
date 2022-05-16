use std::collections::HashMap;

use super::preludes::rocket_prelude::*;
use crate::{
    commit, error_template, rollback, start_transaction,
    utility::{get_list_from_input, GenericError},
};
use sqlx::Executor;

#[get("/new/loan")]
pub async fn get_new_loan() -> Template {
    Template::render(
        "new-loan",
        HashMap::from([("restriction", crate::utility::get_restriction())]),
    )
}

#[derive(Debug, FromForm, Default, Serialize)]
pub struct Submit {
    subbranch: String,
    clientIDs: String,
    amount: String,
}

#[post("/new/loan", data = "<form>")]
pub async fn submit(
    mut db: Connection<BankManage>,
    form: Form<Contextual<'_, Submit>>,
) -> (Status, Template) {
    let status = form.context.status();
    let value = match form.value {
        Some(ref value) => value,
        None => {
            return (
                status,
                error_template!("Error adding loan: failed to receive form"),
            )
        }
    };
    start_transaction!(db);
    let loanID = uuid::Uuid::new_v4().to_string();
    match add_loan_attr(
        &mut db,
        loanID.clone(),
        value.subbranch.clone(),
        value.amount.clone(),
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            rollback!(db);
            return (status, error_template!(e, "Error adding loan attributes"));
        }
    }
    for client_id in get_list_from_input::<Vec<_>>(&value.clientIDs) {
        match add_receiveloan_relation(&mut db, loanID.clone(), client_id).await {
            Ok(_) => (),
            Err(e) => {
                rollback!(db);
                return (
                    status,
                    error_template!(e, "Error adding loan receiving relation"),
                );
            }
        }
    }

    commit!(db);
    (
        status,
        Template::render("new-loan-success", &HashMap::from([("id", loanID)])),
    )
}

/// Modify table `loan`
async fn add_loan_attr(
    db: &mut Connection<BankManage>,
    loanID: String,
    subbranchName: String,
    amount: String,
) -> Result<(), GenericError> {
    sqlx::query(
        "INSERT INTO loan (loanID, subbranchName, amount) VALUES
    (?, ?, ?)
    ",
    )
    .bind(&loanID)
    .bind(&subbranchName)
    .bind(&amount)
    .execute(&mut **db)
    .await?;
    Ok(())
}

async fn add_receiveloan_relation(
    db: &mut Connection<BankManage>,
    loanID: String,
    clientID: String,
) -> Result<(), GenericError> {
    sqlx::query(
        "INSERT INTO receiveLoan (loanID, clientID) VALUES
    (?, ?)
    ",
    )
    .bind(&loanID)
    .bind(&clientID)
    .execute(&mut **db)
    .await?;
    Ok(())
}
