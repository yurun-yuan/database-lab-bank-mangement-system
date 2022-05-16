use crate::{
    error_template,
    utility::{GenericError, Restriction},
};

use super::preludes::rocket_prelude::*;

#[derive(Debug, FromForm, Default, Serialize)]
pub struct Submit {
    clientID: String,
    name: String,
    tel: String,
    address: String,
    contactname: String,
    contactemail: String,
}

#[derive(Serialize)]
pub struct NewClientContext {
    restriction: Restriction,
}

#[get("/new/client")]
pub fn new_client() -> Template {
    Template::render(
        "new-client",
        NewClientContext {
            restriction: crate::utility::get_restriction(),
        },
    )
}

async fn add_client(
    db: &mut Connection<BankManage>,
    new_client: Client,
) -> Result<(), GenericError> {
    sqlx::query(
        "INSERT INTO client (clientID,
                                employeeID,
                                clientName,
                                clientTel,
                                clientAddr,
                                contactName,
                                contactTel,
                                contactEmail,
                                contactRelationship,
                                serviceType) VALUES
                                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(new_client.clientID)
    .bind(new_client.employeeID)
    .bind(new_client.clientName)
    .bind(new_client.clientTel)
    .bind(new_client.clientAddr)
    .bind(new_client.contactName)
    .bind(new_client.contactTel)
    .bind(new_client.contactEmail)
    .bind(new_client.contactRelationship)
    .bind(new_client.serviceType)
    .execute(&mut **db)
    .await?;
    Ok(())
}

#[post("/new/client", data = "<form>")]
pub async fn submit(
    mut db: Connection<BankManage>,
    form: Form<Contextual<'_, Submit>>,
) -> (Status, Template) {
    let template = match form.value {
        Some(ref submission) => {
            let new_client = Client {
                clientID: submission.clientID.clone(),
                clientName: Some(submission.name.clone()),
                clientTel: Some(submission.tel.clone()),
                clientAddr: Some(submission.address.clone()),
                contactName: Some(submission.contactname.clone()),
                ..Client::default()
            };
            match add_client(&mut db, new_client).await {
                Ok(_) => Template::render("new-client-success", &form.context),
                Err(e) => error_template!(e, "Error adding client"),
            }
        }
        None => error_template!("Error adding client: failed to receive form"),
    };

    (form.context.status(), template)
}
