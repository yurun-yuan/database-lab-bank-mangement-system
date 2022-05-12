use crate::schema::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Default, Serialize, PartialEq, Eq, AsChangeset, Hash)]
#[table_name = "client"]
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

#[derive(Queryable, Insertable, Default, Serialize, PartialEq, Eq, AsChangeset, Hash)]
#[table_name = "accountmanagement"]
pub struct AccountManagement {
    pub subbranchName: String,
    pub clientID: String,
    pub savingAccountID: Option<String>,
    pub checkingAccountID: Option<String>,
}
