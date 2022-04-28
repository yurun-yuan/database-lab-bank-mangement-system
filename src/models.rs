use crate::schema::client;

#[derive(Queryable, Insertable, Default)]
#[table_name="client"]
pub struct Client {
    pub clientID: String,
    pub employeeID: Option<String>,
    pub clientName: Option<String>,
    pub clientTel: Option<String>,
    pub clientAddr: Option<String>,
    pub contactName: Option<String>,
    pub contanceTel: Option<String>,
    pub contactEmail: Option<String>,
    pub contactRelationship: Option<String>,
    pub serviceType: Option<String>,
}
