use crate::utility::ErrorContext;
use super::preludes::rocket_prelude::*;
use chrono::prelude::*;
use rocket::futures::TryStreamExt;
use sqlx::{query_as, Executor};

#[derive(Debug, FromForm, Default, Serialize)]
pub struct AccountSubmit {
    pub clientIDs: String,
    pub accountType: String,
    pub currencyType: String,
    pub subbranchName: String,
    pub overdraft: String,
    pub interest: String,
}

#[derive(Debug, FromForm, Default, Serialize)]
pub struct ClientBasicInfoContext {
    id: String,
    name: String,
}

#[get("/new/account")]
pub fn new_account() -> Template {
    Template::render("new-account", &Context::default())
}

#[derive(Debug)]
pub struct AccountConstraintError {}

impl std::fmt::Display for AccountConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "account constraint error")
    }
}

impl std::error::Error for AccountConstraintError {}

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

async fn add_new_account_and_own(
    db: &mut Connection<BankManage>,
    submission: &AccountSubmit,
) -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let account_id = add_account_entity(db, submission).await?;
    let clientIDs = submission.clientIDs.split_whitespace();
    for client_id in clientIDs {
        add_owning_relation(db, client_id.to_string(), account_id.clone(), submission).await?;
    }
    Ok(())
}

/// Add entity to `account`, `savingaccount`/`checkingaccount`
pub async fn add_account_entity(
    db: &mut Connection<BankManage>,
    submission: &AccountSubmit,
) -> Result<String, Box<dyn std::error::Error + Sync + Send + 'static>> {
    let account_id = uuid::Uuid::new_v4().to_string();
    let cur_date = Local::now().format("%Y-%m-%d").to_string();

    let account_id_copy = account_id.clone();
    let cur_date_copy = cur_date.clone();

    // into table `account`
    sqlx::query("insert into account(accountID, balance, openDate) values (?, ?, ?)")
        .bind(account_id_copy)
        .bind(0)
        .bind(cur_date_copy)
        .execute(&mut **db)
        .await?;

    match &submission.accountType as &str {
        "savingAccount" => {
            // into table `savingaccount`
            let account_id_copy = account_id.clone();
            let cur_date_copy = cur_date.clone();
            let interest = submission
                .interest
                .parse::<f64>()
                .expect("Invalid interest");
            let currencyType = submission.currencyType.clone();
            sqlx::query("insert into savingaccount(accountID, balance, openDate, interest, currencyType) values (?, ?, ?, ?, ?)")
                .bind(account_id_copy)
                .bind(0)
                .bind(cur_date_copy)
                .bind(interest)
                .bind(currencyType)
                .execute(&mut **db).await?;
        }
        "checkingAccount" => {
            // into table `checkingaccount`
            let account_id_copy = account_id.clone();
            let cur_date_copy = cur_date.clone();
            let overdraft = submission
                .overdraft
                .parse::<f64>()
                .expect("Invalid overdraft");
            sqlx::query("insert into checkingAccount(accountID, balance, openDate, overdraft) values (?, ?, ?, ?)")
            .bind(account_id_copy)
            .bind(0)
            .bind(cur_date_copy)
            .bind(overdraft)
            .execute(&mut **db).await?;
        }
        _ => {
            return Err(Box::new(AccountConstraintError {}));
        }
    }
    Ok(account_id)
}

/// Add relationship between client and account. Add entity to `own`, `accountmanagement`.
pub async fn add_owning_relation(
    db: &mut Connection<BankManage>,
    client_id: String,
    account_id: String,
    submission: &AccountSubmit,
) -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let cur_time = Local::now().format("%Y-%m-%d %T").to_string();

    // into table `own`
    let account_id_copy = account_id.clone();
    let client_id_copy = client_id.clone();
    sqlx::query("insert into own(accountID, clientID, lastVisitTime) values (?, ?, ?)")
        .bind(account_id_copy)
        .bind(client_id_copy)
        .bind(cur_time)
        .execute(&mut **db)
        .await?;

    // filter in table `accountmanagement`
    let client_id_copy = client_id.clone();
    let subbranchName_copy = submission.subbranchName.clone();
    let account_manage_entry = sqlx::query_as!(
        AccountManagement,
        "SELECT * FROM accountmanagement WHERE subbranchName=? and clientID=?",
        subbranchName_copy,
        client_id_copy
    )
    .fetch_one(&mut **db)
    .await;

    macro_rules! add_to_accountmanagement {
        ($(($account_type: ident, $attr_name: ident)),+) => {
            match  &submission.accountType as &str{
            $(
                stringify!($account_type) => {
                    // into table `accountmanagement`
                    match account_manage_entry {
                        Err(sqlx::Error::RowNotFound) => {
                            let account_manage_entry = AccountManagement {
                                subbranchName: submission.subbranchName.clone(),
                                clientID: client_id.clone(),
                                $attr_name: Some(account_id.clone()),
                                ..AccountManagement::default()
                            };
                            sqlx::query("INSERT INTO accountmanagement (subbranchName,clientID,savingAccountID,checkingAccountID)VALUES(?, ?, ?, ?)")
                            .bind(account_manage_entry.subbranchName)
                            .bind(account_manage_entry.clientID)
                            .bind(account_manage_entry.savingAccountID)
                            .bind(account_manage_entry.checkingAccountID)
                            .execute(&mut **db).await?;
                        }
                        Err(e)=> {return Err(Box::new(e));}
                        Ok(mut account_manage_entry) => {
                            if !account_manage_entry.$attr_name.is_none() {
                                eprintln!("More than one saving account for (client, subbranch)");
                                return Err(Box::new(AccountConstraintError {}));
                            } else {
                                account_manage_entry.$attr_name = Some(account_id.clone());
                                sqlx::query("UPDATE accountmanagement SET
                                savingAccountID=?,
                                checkingAccountID=?
                                WHERE subbranchName=? and clientID=? 
                                ")
                                .bind(account_manage_entry.savingAccountID)
                                .bind(account_manage_entry.checkingAccountID)
                                .bind(account_manage_entry.subbranchName)
                                .bind(account_manage_entry.clientID)
                                .execute(&mut **db).await?;
                            }
                        }
                    }
                }
            )+
            _ => {
            return Err(Box::new(AccountConstraintError {}));
        }
            }
        };
    }

    add_to_accountmanagement!(
        (savingAccount, savingAccountID),
        (checkingAccount, checkingAccountID)
    );
    Ok(())
}
