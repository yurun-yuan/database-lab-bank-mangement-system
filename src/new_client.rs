use super::preludes::diesel_prelude::*;
use super::preludes::rocket_prelude::*;
use super::BMDBConn;

#[derive(Debug, FromForm, Default, Serialize)]
pub struct ClientFromForm {
    name: String,
    tel: String,
    address: String,
    contactname: String,
    contactemail: String,
}

#[derive(Debug, FromForm, Default, Serialize)]
pub struct Submit {
    client: ClientFromForm,
}

#[get("/new/client")]
pub fn new_client() -> Template {
    Template::render("new-client", <Submit as Default>::default())
}

#[post("/new/client", data = "<form>")]
pub async fn submit(conn: BMDBConn, form: Form<Contextual<'_, Submit>>) -> (Status, Template) {
    let template = match form.value {
        Some(ref submission) => {
            eprintln!("submission: {:#?}", submission);

            let new_client = Client {
                clientID: uuid::Uuid::new_v4().to_string(),
                clientName: Some(submission.client.name.clone()),
                clientTel: Some(submission.client.tel.clone()),
                clientAddr: Some(submission.client.address.clone()),
                ..Client::default()
            };

            // diesel::insert_into(client::table).values()

            conn.run(move |conn| {
                diesel::insert_into(client::table)
                    .values(&new_client)
                    .execute(conn)
                    .expect("Error when inserting")
            }).await;

            Template::render("new-client-success", &form.context)
        }
        None => Template::render("index", &form.context),
    };

    (form.context.status(), template)
}
