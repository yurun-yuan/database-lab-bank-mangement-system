use super::preludes::diesel_prelude::*;
use super::preludes::rocket_prelude::*;
use super::BMDBConn;

#[derive(Serialize)]
pub struct ClientProfileContext {
    client: Client,
}

#[get("/profile/client?<id>")]
pub async fn client_profile(conn: BMDBConn, id: String) -> Template {
    let mut client = conn
        .run(move |conn| {
            client::dsl::client
                .filter(client::dsl::clientID.eq(id))
                .limit(1)
                .load::<Client>(conn)
                .expect("Error loading clients")
        })
        .await
        .into_iter();
    let client = client.next();
    match client {
        None => Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("Error querying client: No client found"),
            },
        ),
        Some(client) => Template::render("client-profile", &ClientProfileContext { client }),
    }
}
