use super::preludes::diesel_prelude::*;
use super::preludes::rocket_prelude::*;
use super::BMDBConn;

use chrono::prelude::*;
use diesel::connection::SimpleConnection;

#[derive(Debug, FromForm, Default, Serialize)]
pub struct AccountSubmit {
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

#[get("/new/account?<id>&<name>")]
pub fn new_account(id: String, name: String) -> Template {
    Template::render("new-account", &ClientBasicInfoContext { id, name })
}

#[derive(Debug)]
pub struct AccountConstraintError {}

impl std::fmt::Display for AccountConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "account constraint error")
    }
}

impl std::error::Error for AccountConstraintError {}

async fn add_account(
    conn: &BMDBConn,
    submission: &AccountSubmit,
) -> Result<String, Box<dyn std::error::Error + Sync + Send + 'static>> {
    let account_id = uuid::Uuid::new_v4().to_string();
    let cur_date = Local::now().format("%Y-%m-%d").to_string();

    let account_id_copy = account_id.clone();
    let cur_date_copy = cur_date.clone();

    // into table `account`
    conn.run(move |conn| {
        conn.batch_execute(&format!(
            "insert into account(accountID, balance, openDate) values ('{}', {}, '{}')",
            account_id_copy, 0, cur_date_copy,
        ))
    })
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
            conn.run(move |conn| {
                        conn.batch_execute(&format!(
                            "insert into savingaccount(accountID, balance, openDate, interest, currencyType) values ('{}', {}, '{}', {}, '{}')",
                            account_id_copy,
                            0,
                            cur_date_copy,
                            interest,
                            currencyType
                        ))
                    }).await?;
        }
        "checkingAccount" => {
            // into table `checkingaccount`
            let account_id_copy = account_id.clone();
            let cur_date_copy = cur_date.clone();
            let overdraft = submission
                .overdraft
                .parse::<f64>()
                .expect("Invalid overdraft");
            conn.run(move |conn| {
                        conn.batch_execute(&format!(
                            "insert into checkingAccount(accountID, balance, openDate, overdraft) values ('{}', {}, '{}', {})",
                            account_id_copy,
                            0,
                            cur_date_copy,
                            overdraft
                        ))
                    }).await?;
        }
        _ => {
            return Err(Box::new(AccountConstraintError {}));
        }
    }
    Ok(account_id)
}

async fn add_owning(
    conn: &BMDBConn,
    client_id: String,
    account_id: String,
    submission: &AccountSubmit,
) -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let cur_time = Local::now().format("%Y-%m-%d %T").to_string();

    // into table `own`
    let account_id_copy = account_id.clone();
    let client_id_copy = client_id.clone();
    conn.run(move |conn| {
        conn.batch_execute(&format!(
            "insert into own(accountID, clientID, lastVisitTime) values ('{}', '{}', '{}')",
            account_id_copy, client_id_copy, cur_time,
        ))
    })
    .await?;

    // filter in table `accountmanagement`
    let client_id_copy = client_id.clone();
    let subbranchName_copy = submission.subbranchName.clone();
    let account_manage_entry = conn
        .run(move |conn| {
            accountmanagement::dsl::accountmanagement
                .filter(crate::accountmanagement::subbranchName.eq(subbranchName_copy))
                .filter(crate::accountmanagement::clientID.eq(client_id_copy))
                .limit(1)
                .load::<AccountManagement>(conn)
        })
        .await?
        .into_iter()
        .next();

    macro_rules! add_to_accountmanagement {
        ($(($account_type: ident, $attr_name: ident)),+) => {
            match  &submission.accountType as &str{
            $(
                stringify!($account_type) => {
                    // into table `accountmanagement`
                    match account_manage_entry {
                        None => {
                            let account_manage_entry = AccountManagement {
                                subbranchName: submission.subbranchName.clone(),
                                clientID: client_id.clone(),
                                $attr_name: Some(account_id.clone()),
                                ..AccountManagement::default()
                            };
                            conn.run(move |conn| {
                                diesel::insert_into(accountmanagement::table)
                                    .values(account_manage_entry)
                                    .execute(conn)
                            })
                            .await?;
                        }
                        Some(mut account_manage_entry) => {
                            if !account_manage_entry.$attr_name.is_none() {
                                eprintln!("More than one saving account for (client, subbranch)");
                                return Err(Box::new(AccountConstraintError {}));
                            } else {
                                account_manage_entry.$attr_name = Some(account_id.clone());
                                conn.run(move |conn| {
                                    diesel::update(accountmanagement::table)
                                        .filter(
                                            crate::accountmanagement::subbranchName
                                                .eq(account_manage_entry.subbranchName.clone()),
                                        )
                                        .filter(
                                            crate::accountmanagement::clientID
                                                .eq(account_manage_entry.clientID.clone()),
                                        )
                                        .set(&account_manage_entry)
                                        .execute(conn)
                                })
                                .await?;
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

async fn add_new_account_and_own(
    conn: &BMDBConn,
    client_id: String,
    submission: &AccountSubmit,
) -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let account_id = add_account(conn, submission).await?;
    add_owning(conn, client_id, account_id, submission).await?;
    Ok(())
}

#[post("/new/account?<id>", data = "<form>")]
pub async fn submit(
    conn: BMDBConn,
    id: String,
    form: Form<Contextual<'_, AccountSubmit>>,
) -> (Status, Template) {
    let template;
    match form.value {
        Some(ref submission) => {
            conn.run(move |conn| conn.batch_execute(&format!("START TRANSACTION")))
                .await
                .expect("Error adding account");
            let result = add_new_account_and_own(&conn, id.clone(), submission).await;
            match result {
                Ok(()) => template = Template::render("new-account-success", &form.context),
                Err(e) => {
                    conn.run(move |conn| conn.batch_execute(&format!("ROLLBACK")))
                        .await
                        .expect(&format!(
                            "Rollback with error {e_info}",
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
            conn.run(move |conn| conn.batch_execute(&format!("COMMIT")))
                .await
                .expect("Error adding account");
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
