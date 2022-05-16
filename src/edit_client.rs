use std::collections::HashMap;

use crate::{error_template, utility::{Restriction, validate_string_value}};

use super::preludes::rocket_prelude::*;

#[derive(Serialize)]
pub struct ClientProfileContext {
    client: Client,
    restriction: Restriction,
    service_type: (bool, bool, bool),
}

#[get("/edit/client?<id>")]
pub async fn get_edit_client(mut db: Connection<BankManage>, id: String) -> Template {
    match super::client_profile::query_client_by_id(&mut db, id).await {
        Ok(client) => {
            let service_type = match client.serviceType {
                None => (false, false, true),
                Some(ref service_type) => {
                    if service_type == "account" {
                        (true, false, false)
                    } else if service_type == "loan" {
                        (false, true, false)
                    } else {
                        return error_template!("Unexpected service type");
                    }
                }
            };
            Template::render(
                "edit-client",
                &ClientProfileContext {
                    client,
                    restriction: crate::utility::get_restriction(),
                    service_type,
                },
            )
        }
        Err(e) => error_template!(e, "Error loading client"),
    }
}

#[derive(Debug, FromForm, Default, Serialize, Clone)]
pub struct ClientFromForm {
    pub clientName: String,
    pub clientTel: String,
    pub clientAddr: String,
    pub contactName: String,
    pub contactEmail: String,
    pub employeeID: String,
    pub contactTel: String,
    pub contactRelationship: String,
    pub serviceType: String,
}

#[post("/edit/client?<id>", data = "<form>")]
pub async fn act_edit_client(
    mut db: Connection<BankManage>,
    id: String,
    form: Form<Contextual<'_, ClientFromForm>>,
) -> (Status, Template) {
    let template;
    if let Some(submission) = form.value.clone() {
        let update_result = sqlx::query(&format!(
            "UPDATE client SET 
                employeeID={},
                clientName = {},
                clientTel = {},
                clientAddr = {},
                contactName = {},
                contactTel={},
                contactEmail = {},
                contactRelationship={},
                serviceType={}
                WHERE clientID={}
                ",
            validate_string_value(&submission.employeeID),
            validate_string_value(&submission.clientName),
            validate_string_value(&submission.clientTel),
            validate_string_value(&submission.clientAddr),
            validate_string_value(&submission.contactName),
            validate_string_value(&submission.contactTel),
            validate_string_value(&submission.contactEmail),
            validate_string_value(&submission.contactRelationship),
            validate_string_value(&submission.serviceType),
            validate_string_value(&id),
        ))
        .execute(&mut *db)
        .await;
        template = match update_result {
            Ok(_) => Template::render("update-client-success", HashMap::from([("id", id)])),
            Err(e) => error_template!(e, "Error updating client"),
        };
    } else {
        template = error_template!("Error receiving form");
    };

    (form.context.status(), template)
}

#[get("/delete/client?<id>")]
pub async fn delete_client(mut db: Connection<BankManage>, id: String) -> Template {
    eprintln!("delete {id}");
    match sqlx::query("delete from client where clientID=?")
        .bind(id)
        .execute(&mut *db)
        .await
    {
        Ok(_) => Template::render("delete-client-success", &Context::default()),
        Err(e) => error_template!(e, "Error deleting client"),
    }
}
