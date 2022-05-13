use chrono::NaiveDate;
use serde::Serialize;
use sqlx::types::Decimal;

// use crate::utility::RawSqlDataString;

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

#[derive(Default, Serialize, PartialEq, Eq, Hash)]
pub struct AccountManagement {
    pub subbranchName: String,
    pub clientID: String,
    pub savingAccountID: Option<String>,
    pub checkingAccountID: Option<String>,
}

#[derive(PartialEq, sqlx::FromRow, Debug)]
pub struct Account {
    pub accountID: String,
    pub balance: sqlx::types::BigDecimal,
    pub openDate: NaiveDate,
}

#[derive(PartialEq, sqlx::FromRow, Debug)]
pub struct SavingAccount {
    pub accountID: String,
    pub balance: Option<sqlx::types::BigDecimal>,
    pub openDate: Option<NaiveDate>,
    pub interest: Option<f32>,
    pub currencyType: Option<String>,
}

#[derive(PartialEq, sqlx::FromRow, Debug)]
pub struct CheckingAccount {
    pub accountID: String,
    pub balance: Option<sqlx::types::BigDecimal>,
    pub openDate: Option<NaiveDate>,
    pub overdraft: Option<sqlx::types::BigDecimal>,
}
