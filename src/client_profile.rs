use rocket::futures::TryStreamExt;

use crate::utility::GenericError;

use super::preludes::rocket_prelude::*;
use std::vec;

#[derive(Serialize)]
pub struct ClientProfileContext {
    client: Client,
    accounts: Vec<String>,
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

#[get("/profile/client?<id>")]
pub async fn client_profile(mut db: Connection<BankManage>, id: String) -> Template {
    let accounts = query_associated_accounts(&mut db, id.clone())
        .await
        .unwrap_or(vec![]);
    match query_client_by_id(&mut db, id.clone()).await {
        Ok(client) => {
            Template::render("client-profile", &ClientProfileContext { client, accounts })
        }
        Err(e) => Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("Error querying client: {}", e.to_string()),
            },
        ),
    }
}
