use std::collections::HashMap;

use sqlx::Executor;

use super::preludes::rocket_prelude::*;
use crate::{
    account_manage::{delete::*, update::*},
    commit, error_template, rollback, start_transaction,
    utility::get_list_from_input,
};

#[get("/edit/account?<id>")]
pub async fn get_edit_account(mut db: Connection<BankManage>, id: String) -> Template {
    use super::account_manage::query::*;
    let clients = match query_associated_clients(&mut db, id.clone()).await {
        Err(e) => return error_template!(e, "Error fetching account info"),
        Ok(result) => result
            .into_iter()
            .fold(String::new(), |joined, cur| joined + " " + &cur),
    };
    match query_account_by_id(&mut db, id).await {
        Err(e) => return error_template!(e, "Error fetching account info"),
        Ok((specific_account, _)) => match specific_account {
            SpecificAccount::SavingAccount(saving_account) => {
                eprintln!("saving account: {saving_account:?}");
                Template::render(
                    "edit-saving-account",
                    HashMap::from([
                        ("id".to_string(), saving_account.accountID),
                        ("clientIDs".to_string(), clients),
                        (
                            "balance".to_string(),
                            saving_account.balance.unwrap().to_string(),
                        ),
                        (
                            "currencyType".to_string(),
                            saving_account
                                .currencyType
                                .unwrap_or_else(|| "None".to_string()),
                        ),
                        (
                            "interest".to_string(),
                            saving_account.interest.unwrap_or(0f32).to_string(),
                        ),
                    ]),
                )
            }
            SpecificAccount::CheckingAccount(checking_account) => Template::render(
                "edit-checking-account",
                HashMap::from([
                    ("id".to_string(), checking_account.accountID),
                    ("clientIDs".to_string(), clients),
                    (
                        "balance".to_string(),
                        checking_account.balance.unwrap().to_string(),
                    ),
                    (
                        "overdraft".to_string(),
                        checking_account.overdraft.unwrap().to_string(),
                    ),
                ]),
            ),
        },
    }
}

#[post("/edit/account/saving?<id>", data = "<form>")]
pub async fn act_edit_saving_account(
    mut db: Connection<BankManage>,
    id: String,
    form: Form<Contextual<'_, SavingAccountSubmit>>,
) -> (Status, Template) {
    if form.value.is_none() {
        return (
            form.context.status(),
            error_template!("Error receiving form"),
        );
    }
    let submission = form.value.as_ref().unwrap();
    start_transaction!(db);
    let updated_associated_client_IDs: std::collections::HashSet<String> =
        get_list_from_input(&submission.clientIDs);
    match update_saving_account_and_own(
        &mut db,
        id.clone(),
        submission.clone(),
        updated_associated_client_IDs,
    )
    .await
    {
        Ok(_) => {
            commit!(db);
            (
                form.context.status(),
                Template::render("update-account-success", HashMap::from([("id", id)])),
            )
        }
        Err(e) => {
            rollback!(db);
            (
                form.context.status(),
                error_template!(e, "Error updating account"),
            )
        }
    }
}

#[post("/edit/account/checking?<id>", data = "<form>")]
pub async fn act_edit_checking_account(
    mut db: Connection<BankManage>,
    id: String,
    form: Form<Contextual<'_, CheckingAccountSubmit>>,
) -> (Status, Template) {
    if form.value.is_none() {
        return (
            form.context.status(),
            error_template!("Error receiving form"),
        );
    }
    let submission = form.value.as_ref().unwrap();
    start_transaction!(db);
    let updated_associated_client_IDs: std::collections::HashSet<String> =
        get_list_from_input(&submission.clientIDs);
    match update_checking_account_and_own(
        &mut db,
        id.clone(),
        submission.clone(),
        updated_associated_client_IDs,
    )
    .await
    {
        Ok(_) => {
            commit!(db);
            (
                form.context.status(),
                Template::render("update-account-success", HashMap::from([("id", id)])),
            )
        }
        Err(e) => {
            rollback!(db);
            (
                form.context.status(),
                error_template!(e, "Error updating account"),
            )
        }
    }
}

#[get("/delete/account?<id>")]
pub async fn delete_account(mut db: Connection<BankManage>, id: String) -> Template {
    start_transaction!(db);
    let template = match delete_account_and_own(&mut db, id).await {
        Ok(_) => {
            commit!(db);
            Template::render("delete-account-success", &Context::default())
        }
        Err(e) => {
            rollback!(db);
            error_template!(e, "Error deleting account")
        }
    };

    template
}
