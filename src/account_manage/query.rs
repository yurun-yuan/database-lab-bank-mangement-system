use crate::{preludes::rocket_prelude::*, utility::GenericError};
use rocket::futures::TryStreamExt;

#[derive(Clone, Debug)]
pub enum SpecificAccount {
    SavingAccount(SavingAccount),
    CheckingAccount(CheckingAccount),
}

/// Returns (SpecificAccount, subbranchName).
pub async fn query_account_by_id(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<(SpecificAccount, String), GenericError> {
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
                    Ok((SpecificAccount::SavingAccount(saving_account), subbranch))
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
                            Ok((
                                SpecificAccount::CheckingAccount(checking_account),
                                subbranch,
                            ))
                        }
                        Err(e) => Err(Box::new(e)),
                    }
                }
                Err(sqlx::Error::RowNotFound) => Err(Box::new(sqlx::Error::RowNotFound)),
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

/// Returns the list of client ids the client of which is associated
pub async fn query_associated_clients(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<Vec<String>, GenericError> {
    let mut result = vec![];
    let mut rows = sqlx::query("SELECT clientID FROM own WHERE accountID=?")
        .bind(id)
        .fetch(&mut **db);
    while let Some(row) = rows.try_next().await? {
        result.push(row.try_get::<'_, String, _>(0)?);
    }
    Ok(result)
}
