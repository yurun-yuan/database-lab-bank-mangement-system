use sqlx::Executor;

use super::preludes::rocket_prelude::*;
use crate::{
    account_manage::{delete::*, insert::*, query::*},
    utility::ErrorContext,
};

#[derive(Serialize)]
pub struct ClientProfileContext {
    client: Client,
}

#[get("/edit/account?<id>")]
pub async fn get_edit_account(mut db: Connection<BankManage>, id: String) -> Template {
    todo!()
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

#[post("/edit/account?<id>", data = "<form>")]
pub async fn act_edit_account(
    mut db: Connection<BankManage>,
    id: String,
    form: Form<Contextual<'_, ProfileUpdateSubmit>>,
) -> (Status, Template) {
    todo!();
    let template;
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

#[get("/delete/account?<id>")]
pub async fn delete_account(mut db: Connection<BankManage>, id: String) -> Template {
    let template;
    db.execute("START TRANSACTION")
        .await
        .expect("Error starting a transaction");
    match delete_account_and_own(&mut db, id).await {
        Ok(_) => template = Template::render("delete-account-success", &Context::default()),
        Err(e) => {
            db.execute("ROLLBACK").await.expect(&format!(
                "Error rolling back: {e_info}",
                e_info = e.to_string()
            ));
            template = Template::render(
                "error",
                &ErrorContext {
                    info: format!("Error deleting account: {e_info}", e_info = e.to_string()),
                },
            )
        }
    }
    db.execute("COMMIT").await.expect("Error committing");
    template
}
