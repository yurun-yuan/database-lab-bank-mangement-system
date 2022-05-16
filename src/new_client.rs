use crate::{
    error_template,
    utility::{validate_string_value, GenericError, Restriction},
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
    contactrelation: String,
    contacttel: String,
    employeeID: String,
    servicetype: String,
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

// NOTE the attributes of new_client should not be None
async fn add_client(
    db: &mut Connection<BankManage>,
    new_client: Client,
) -> Result<(), GenericError> {
    sqlx::query(&format!(
        "INSERT INTO client (clientID,
            clientName,
            clientTel,
            clientAddr,
            contactName,
            contactTel,
            contactEmail,
            contactRelationship,
            employeeID,
            serviceType
            ) VALUES
            ({}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
        validate_string_value(&new_client.clientID),
        validate_string_value(&new_client.clientName.unwrap()),
        validate_string_value(&new_client.clientTel.unwrap()),
        validate_string_value(&new_client.clientAddr.unwrap()),
        validate_string_value(&new_client.contactName.unwrap()),
        validate_string_value(&new_client.contactTel.unwrap()),
        validate_string_value(&new_client.contactEmail.unwrap()),
        validate_string_value(&new_client.contactRelationship.unwrap()),
        validate_string_value(&new_client.employeeID.unwrap()),
        validate_string_value(&new_client.serviceType.unwrap()),
    ))
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
                employeeID: Some(submission.employeeID.clone()),
                contactTel: Some(submission.contacttel.clone()),
                contactEmail: Some(submission.contactemail.clone()),
                contactRelationship: Some(submission.contactrelation.clone()),
                serviceType: Some(submission.servicetype.clone()),
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
