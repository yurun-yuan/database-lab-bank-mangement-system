use super::preludes::diesel_prelude::*;
use super::preludes::rocket_prelude::*;
use super::BMDBConn;

#[derive(Serialize)]
pub struct ClientProfileContext {
    client: Client,
}

#[get("/edit/client?<id>")]
pub async fn get_edit_client(conn: BMDBConn, id: String) -> Template {
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
        None => {
            Template::render("error", &Context::default());
            todo!()
        }
        Some(client) => Template::render("edit-client", &ClientProfileContext { client }),
    }
}

#[derive(Debug, FromForm, Default, Serialize, AsChangeset, Clone)]
#[table_name = "client"]
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
    conn: BMDBConn,
    id: String,
    form: Form<Contextual<'_, ProfileUpdateSubmit>>,
) -> (Status, Template) {
    let template;
    let id_copy = id.clone();
    if let Some(submission) = form.value.clone() {
        conn.run(move |conn| {
            diesel::update(client::table)
                .filter(client::dsl::clientID.eq(id_copy))
                .set(&submission.client)
                .execute(conn)
                .expect("Error occurs when updating");
        })
        .await;

        template = Template::render("update-client-success", &SuccessContext { id });
    } else {
        template = Template::render("error", &form.context);
    };

    (form.context.status(), template)
}

#[get("/delete/client?<id>")]
pub async fn delete_client(conn: BMDBConn, id: String) -> Template {
    match conn
        .run(move |conn| {
            diesel::delete(client::table)
                .filter(client::dsl::clientID.eq(id))
                .execute(conn)
        })
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
