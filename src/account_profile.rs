use crate::{client_profile::query_client_by_id, utility::ErrorContext};
use rocket::futures::TryStreamExt;

use super::preludes::rocket_prelude::*;

#[derive(Serialize)]
pub struct AccountProfileContext {
    accountID: String,
    balance: String,
    openDate: String,
    subbranch: String,
    associated_clients: Vec<String>,
    account_type: String,
    details: Vec<(String, String)>,
}

enum SpecificAccount {
    SavingAccount(SavingAccount),
    CheckingAccount(CheckingAccount),
}

async fn query_account_by_id(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<(SpecificAccount, String), Box<dyn std::error::Error>> {
    let saving_account_result = sqlx::query_as!(
        SavingAccount,
        "SELECT * FROM savingAccount WHERE accountID=?",
        id
    )
    .fetch_one(&mut **db)
    .await;
    match saving_account_result {
        Ok(saving_account) => {
            match sqlx::query("SELECT subbranchName FROM accountmanagement WHERE savingAccountID=?")
                .bind(id)
                .fetch_one(&mut **db)
                .await?
                .try_get::<'_, String, _>(0)
            {
                Ok(subbranch) => {
                    return Ok((SpecificAccount::SavingAccount(saving_account), subbranch))
                }
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            let checking_account_result = sqlx::query_as!(
                CheckingAccount,
                "SELECT * FROM checkingAccount WHERE accountID=?",
                id
            )
            .fetch_one(&mut **db)
            .await;
            match checking_account_result {
                Ok(checking_account) => {
                    match sqlx::query(
                        "SELECT subbranchName FROM accountmanagement WHERE checkingAccountID=?",
                    )
                    .bind(id)
                    .fetch_one(&mut **db)
                    .await?
                    .try_get::<'_, String, _>(0)
                    {
                        Ok(subbranch) => {
                            return Ok((
                                SpecificAccount::CheckingAccount(checking_account),
                                subbranch,
                            ))
                        }
                        Err(e) => Err(Box::new(e)),
                    }
                }
                Err(sqlx::Error::RowNotFound) => return Err(Box::new(sqlx::Error::RowNotFound)),
                Err(e) => return Err(Box::new(e)),
            }
        }
        Err(e) => return Err(Box::new(e)),
    }
}

async fn query_associated_clients(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result = vec![];
    let mut rows = sqlx::query("SELECT clientID FROM own WHERE accountID=?")
        .bind(id)
        .fetch(&mut **db);
    while let Some(row) = rows.try_next().await? {
        result.push(row.try_get::<'_, String, _>(0)?);
    }
    Ok(result)
}

#[get("/profile/account?<id>")]
pub async fn client_profile(mut db: Connection<BankManage>, id: String) -> Template {
    let associated_clients;
    match query_associated_clients(&mut db, id.clone()).await {
        Ok(clients) => {
            associated_clients = clients;
        }
        Err(e) => {
            return Template::render(
                "error",
                &ErrorContext {
                    info: e.to_string(),
                },
            )
        }
    }
    match query_account_by_id(&mut db, id.clone()).await {
        Ok((specific_account, subbranch)) => match specific_account {
            SpecificAccount::SavingAccount(saving_account) => Template::render(
                "account-profile",
                AccountProfileContext {
                    accountID: saving_account.accountID,
                    balance: saving_account.balance.unwrap().to_string(),
                    openDate: saving_account.openDate.unwrap().to_string(),
                    subbranch,
                    account_type: "saving account".to_string(),
                    details: vec![
                        (
                            "Interest".to_string(),
                            saving_account.interest.unwrap_or(0f32).to_string(),
                        ),
                        (
                            "Currency type".to_string(),
                            saving_account.currencyType.unwrap_or("None".to_string()),
                        ),
                    ],
                    associated_clients,
                },
            ),
            SpecificAccount::CheckingAccount(checking_account) => Template::render(
                "account-profile",
                AccountProfileContext {
                    accountID: checking_account.accountID,
                    balance: checking_account.balance.unwrap().to_string(),
                    openDate: checking_account.openDate.unwrap().to_string(),
                    subbranch,
                    account_type: "checking account".to_string(),
                    details: vec![(
                        "overdraft".to_string(),
                        checking_account.overdraft.unwrap().to_string(),
                    )],
                    associated_clients,
                },
            ),
        },
        Err(e) => Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("Error querying client: {}", e.to_string()),
            },
        ),
    }
}
