use super::preludes::rocket_prelude::*;

#[derive(Serialize)]
pub struct ClientProfileContext {
    client: Client,
}

#[get("/edit/client?<id>")]
pub async fn get_edit_client(mut db: Connection<BankManage>, id: String) -> Template {
    match super::client_profile::query_client_by_id(&mut db, id).await {
        Ok(client) => Template::render("edit-client", &ClientProfileContext { client }),
        Err(e) => Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("Error loading client: {}", e.to_string()),
            },
        ),
    }
}

#[derive(Debug, FromForm, Default, Serialize, Clone)]
pub struct ClientFromForm {
    pub clientName: String,
    pub clientTel: String,
    pub clientAddr: String,
    pub contactName: String,
    pub contactEmail: String,
}

#[derive(Debug, FromForm, Default, Serialize, Clone)]
pub struct ProfileUpdateSubmit {
    client: ClientFromForm,
}

#[derive(Serialize)]
pub struct SuccessContext {
    id: String,
}

#[post("/edit/client?<id>", data = "<form>")]
pub async fn act_edit_client(
    mut db: Connection<BankManage>,
    id: String,
    form: Form<Contextual<'_, ProfileUpdateSubmit>>,
) -> (Status, Template) {
    let template;
    let id_copy = id.clone();
    if let Some(submission) = form.value.clone() {
        let update_result = sqlx::query(
            "UPDATE client SET 

                clientName = ?,
                clientTel = ?,
                clientAddr = ?,
                contactName = ?,
                
                contactEmail = ?
                
                WHERE clientID=?
                ",
        )
        // .bind(submission.client.employeeID)
        .bind(submission.client.clientName)
        .bind(submission.client.clientTel)
        .bind(submission.client.clientAddr)
        .bind(submission.client.contactName)
        // .bind(submission.client.contactTel)
        .bind(submission.client.contactEmail)
        // .bind(submission.client.contactRelationship)
        // .bind(submission.client.serviceType)
        .bind(id.clone())
        .execute(&mut *db)
        .await;
        match update_result {
            Ok(_) => template = Template::render("update-client-success", &SuccessContext { id }),
            Err(e) => {
                template = Template::render(
                    "error",
                    &crate::utility::ErrorContext {
                        info: format!("Error updating client: {}", e.to_string()),
                    },
                )
            }
        }
    } else {
        template = Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("Error receiving form"),
            },
        );
    };

    (form.context.status(), template)
}

#[get("/delete/client?<id>")]
pub async fn delete_client(mut db: Connection<BankManage>, id: String) -> Template {
    eprintln!("delete {id}");
    // match db
    //     .run(move |db| db.batch_execute(&format!("delete from client where clientID={}", id)))
    //     .await
    match sqlx::query("delete from client where clientID=?")
        .bind(id)
        .execute(&mut *db)
        .await
    {
        Ok(_) => Template::render("delete-client-success", &Context::default()),
        Err(e) => Template::render(
            "error",
            &crate::utility::ErrorContext {
                info: format!("Error deleting client: {e_info}", e_info = e.to_string()),
            },
        ),
    }
}
