use super::preludes::rocket_prelude::*;
use crate::{error_template, utility::GenericError};
use rocket::futures::TryStreamExt;
use std::vec;

#[derive(Serialize)]
pub struct ClientProfileContext {
    client: Client,
    accounts: Vec<String>,
    loans: Vec<String>,
}

pub async fn query_client_by_id(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<Client, GenericError> {
    Ok(
        sqlx::query_as!(Client, "SELECT * FROM client WHERE clientID=?", id)
            .fetch_one(&mut **db)
            .await?,
    )
}

async fn query_associated_accounts(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<Vec<String>, GenericError> {
    let mut result = vec![];
    let mut rows = sqlx::query("SELECT accountID FROM own WHERE clientID=?")
        .bind(id)
        .fetch(&mut **db);
    while let Some(row) = rows.try_next().await? {
        result.push(row.try_get::<'_, String, _>(0)?);
    }
    Ok(result)
}

async fn query_associated_loans(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<Vec<String>, GenericError> {
    let mut result = vec![];
    let mut rows = sqlx::query("SELECT loanID FROM receiveLoan WHERE clientID=?")
        .bind(id)
        .fetch(&mut **db);
    while let Some(row) = rows.try_next().await? {
        result.push(row.try_get::<'_, String, _>(0)?);
    }
    Ok(result)
}

#[get("/profile/client?<id>")]
pub async fn client_profile(mut db: Connection<BankManage>, id: String) -> Template {
    let accounts = query_associated_accounts(&mut db, id.clone())
        .await
        .unwrap_or_default();
    let loans = query_associated_loans(&mut db, id.clone())
        .await
        .unwrap_or_default();
    match query_client_by_id(&mut db, id.clone()).await {
        Ok(client) => Template::render(
            "client-profile",
            &ClientProfileContext {
                client,
                loans,
                accounts,
            },
        ),
        Err(e) => error_template!(e, "Error querying client"),
    }
}
