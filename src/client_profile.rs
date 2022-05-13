use super::preludes::rocket_prelude::*;

#[derive(Serialize)]
pub struct ClientProfileContext {
    client: Client,
}

pub async fn query_client_by_id(
    mut db: Connection<BankManage>,
    id: String,
) -> Result<Client, Box<dyn std::error::Error>> {
    Ok(
        sqlx::query_as!(Client, "SELECT * FROM client WHERE clientID=?", id)
            .fetch_one(&mut *db)
            .await?,
    )
}

#[get("/profile/client?<id>")]
pub async fn client_profile(db: Connection<BankManage>, id: String) -> Template {
    match query_client_by_id(db, id).await {
        Ok(client) => Template::render("client-profile", &ClientProfileContext { client }),
        Err(e) => Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("Error querying client: {}", e.to_string()),
            },
        ),
    }
}
