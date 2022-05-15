use crate::{preludes::rocket_prelude::*, utility::GenericError};

use super::query::SpecificAccount;

#[derive(Clone, Debug)]
pub enum AccountType {
    SavingAccount,
    CheckingAccount,
}

impl From<&SpecificAccount> for AccountType {
    fn from(specific_account: &SpecificAccount) -> Self {
        match specific_account {
            SpecificAccount::SavingAccount(_) => AccountType::SavingAccount,
            SpecificAccount::CheckingAccount(_) => AccountType::CheckingAccount,
        }
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::SavingAccount => write!(f, "savingAccount"),
            AccountType::CheckingAccount => write!(f, "checkingAccount"),
        }
    }
}

pub async fn delete_account_and_own(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<(), GenericError> {
    let account = super::query::query_account_by_id(db, &id).await?;
    eprintln!("Start deleting account {id}, {account:?}");
    let associated_clients = super::query::query_associated_clients(db, id.clone()).await?;

    for client_id in associated_clients {
        delete_owning_relation(
            db,
            client_id,
            match account.0 {
                SpecificAccount::SavingAccount(ref a) => a.accountID.clone(),
                SpecificAccount::CheckingAccount(ref a) => a.accountID.clone(),
            },
            AccountType::from(&account.0),
            account.1.clone(),
        )
        .await?;
    }

    eprintln!("own relations removed");

    delete_account_entity(db, id.clone(), AccountType::from(&account.0)).await?;

    Ok(())
}

pub async fn delete_account_entity(
    db: &mut Connection<BankManage>,
    id: String,
    account_type: AccountType,
) -> Result<(), GenericError> {
    // delete from saving/checking account
    let to_update_account_table = match account_type {
        AccountType::SavingAccount => "savingaccount",
        AccountType::CheckingAccount => "checkingaccount",
    };
    sqlx::query(&format!(
        "DELETE FROM {to_update_account_table} WHERE accountID=?"
    ))
    .bind(&id)
    .execute(&mut **db)
    .await?;

    eprintln!("specific account {to_update_account_table} removed {id}");

    // delete from `account`
    sqlx::query("DELETE FROM account WHERE accountID=?")
        .bind(&id)
        .execute(&mut **db)
        .await?;
    eprintln!("generic account removed");
    Ok(())
}

pub async fn delete_owning_relation(
    db: &mut Connection<BankManage>,
    client_id: String,
    account_id: String,
    account_type: AccountType,
    subbranch: String,
) -> Result<(), GenericError> {
    // delete from table `own`
    sqlx::query("DELETE FROM own WHERE clientID=? and accountID=?")
        .bind(&client_id)
        .bind(&account_id)
        .execute(&mut **db)
        .await?;

    eprintln!("removed own of client {client_id}");

    // update `accountmanagement`
    let updated_account_id = match account_type {
        AccountType::SavingAccount => "savingAccountID",
        AccountType::CheckingAccount => "checkingAccountID",
    };

    sqlx::query(&format!("UPDATE accountmanagement SET {updated_account_id}=NULL WHERE subbranchName=? and clientID=?")).bind(&subbranch).bind(&client_id).execute(&mut **db).await?;

    // remove `accountmanagement` entry if necessary
    let account_manage_entry = sqlx::query_as!(
        AccountManagement,
        "SELECT * FROM accountmanagement WHERE subbranchName=? and clientID=?",
        subbranch,
        client_id
    )
    .fetch_one(&mut **db)
    .await?;

    if account_manage_entry.checkingAccountID.is_none()
        && account_manage_entry.savingAccountID.is_none()
    {
        sqlx::query("DELETE FROM accountmanagement WHERE subbranchName=? and clientID=?")
            .bind(&subbranch)
            .bind(&client_id)
            .execute(&mut **db)
            .await?;
    }

    Ok(())
}
