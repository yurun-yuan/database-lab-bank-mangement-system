use serde::Serialize;
use sqlx::Row;

#[derive(Default, Serialize, PartialEq, Eq, Hash, sqlx::FromRow)]
pub struct Client {
    pub clientID: String,
    pub employeeID: Option<String>,
    pub clientName: Option<String>,
    pub clientTel: Option<String>,
    pub clientAddr: Option<String>,
    pub contactName: Option<String>,
    pub contactTel: Option<String>,
    pub contactEmail: Option<String>,
    pub contactRelationship: Option<String>,
    pub serviceType: Option<String>,
}

pub fn row_to_client(row: sqlx::mysql::MySqlRow) -> Result<Client, Box<dyn std::error::Error>> {
    Ok(Client {
        clientID: row.try_get("clientID")?,
        employeeID: Some(row.try_get("employeeID")?),
        clientName: Some(row.try_get("clientName")?),
        clientTel: Some(row.try_get("clientTel")?),
        clientAddr: Some(row.try_get("clientAddr")?),
        contactName: Some(row.try_get("contactName")?),
        contactTel: Some(row.try_get("contactTel")?),
        contactEmail: Some(row.try_get("contactEmail")?),
        contactRelationship: Some(row.try_get("contactRelationship")?),
        serviceType: Some(row.try_get("serviceType")?),
    })
}

#[derive(Default, Serialize, PartialEq, Eq, Hash)]
pub struct AccountManagement {
    pub subbranchName: String,
    pub clientID: String,
    pub savingAccountID: Option<String>,
    pub checkingAccountID: Option<String>,
}

#[derive(PartialEq)]
pub struct Account {
    pub accountID: String,
    pub balance: f64,
    pub openDate: chrono::NaiveDate,
}
