use std::collections::HashSet;

use crate::{preludes::rocket_prelude::*, utility::GenericError};
use rocket::futures::TryStreamExt;
use sqlx::types::BigDecimal;

use super::{
    delete::AccountType,
    query::{query_account_by_id, SpecificAccount},
};

// Modify table `account`
async fn update_generic_account(
    db: &mut Connection<BankManage>,
    id: String,
    balance: &String,
) -> std::result::Result<(), GenericError> {
    sqlx::query("UPDATE account SET balance=? WHERE accountID=?")
        .bind(&balance)
        .bind(&id)
        .execute(&mut **db)
        .await?;
    Ok(())
}

#[derive(Debug, FromForm, Default, Serialize, Clone)]
pub struct SavingAccountSubmit {
    pub clientIDs: String,
    pub balance: String,
    pub currencyType: String,
    pub interest: String,
}

// Modify table `saving account` and `account`
async fn update_saving_account(
    db: &mut Connection<BankManage>,
    id: String,
    new: SavingAccountSubmit,
) -> std::result::Result<(), GenericError> {
    update_generic_account(db, id.clone(), &new.balance.clone()).await?;
    sqlx::query(
        "UPDATE savingaccount SET 
        balance=?,
        interest=?,
        currencyType=?
        WHERE accountID=?
    ",
    )
    .bind(&new.balance)
    .bind(&new.interest)
    .bind(&new.currencyType)
    .bind(&id)
    .execute(&mut **db)
    .await?;
    Ok(())
}

pub async fn update_saving_account_and_own(
    db: &mut Connection<BankManage>,
    id: String,
    new: SavingAccountSubmit,
    new_associated_client_IDs: HashSet<String>,
) -> std::result::Result<(), GenericError> {
    let (_, subbranch) = query_account_by_id(db, id.clone()).await?;
    update_owning_relation(
        db,
        id.clone(),
        AccountType::SavingAccount,
        subbranch.clone(),
        new_associated_client_IDs,
    )
    .await?;
    update_saving_account(db, id.clone(), new).await?;
    Ok(())
}

#[derive(Debug, FromForm, Default, Serialize, Clone)]
pub struct CheckingAccountSubmit {
    pub clientIDs: String,
    pub balance: String,
    pub overdraft: String,
}

// Modify table `checking account` and `account`
async fn update_checking_account(
    db: &mut Connection<BankManage>,
    id: String,
    new: CheckingAccountSubmit,
) -> std::result::Result<(), GenericError> {
    update_generic_account(db, id.clone(), &new.balance.clone()).await?;
    sqlx::query(
        "UPDATE checkingaccount SET 
        balance=?,
        overdraft=?
        WHERE accountID=?
    ",
    )
    .bind(&new.balance)
    .bind(&new.overdraft)
    .bind(&id)
    .execute(&mut **db)
    .await?;
    Ok(())
}

pub async fn update_checking_account_and_own(
    db: &mut Connection<BankManage>,
    id: String,
    new: CheckingAccountSubmit,
    new_associated_client_IDs: HashSet<String>,
) -> std::result::Result<(), GenericError> {
    let (_, subbranch) = query_account_by_id(db, id.clone()).await?;
    update_owning_relation(
        db,
        id.clone(),
        AccountType::CheckingAccount,
        subbranch.clone(),
        new_associated_client_IDs,
    )
    .await?;
    update_checking_account(db, id.clone(), new).await?;
    Ok(())
}

pub async fn update_owning_relation(
    db: &mut Connection<BankManage>,
    id: String,
    account_type: AccountType,
    subbranch: String,
    new_associated_client_IDs: HashSet<String>,
) -> Result<(), GenericError> {
    let current_associated_client_IDs: HashSet<_> =
        super::query::query_associated_clients(db, id.clone())
            .await?
            .into_iter()
            .collect();
    let to_add = new_associated_client_IDs.difference(&current_associated_client_IDs);
    let to_remove = current_associated_client_IDs.difference(&new_associated_client_IDs);

    use super::insert::*;

    for to_add_client in to_add {
        add_owning_relation(
            db,
            to_add_client.to_string(),
            id.clone(),
            account_type.to_string(),
            subbranch.clone(),
        )
        .await?;
    }

    use super::delete::*;

    for to_remove_client in to_remove {
        delete_owning_relation(
            db,
            to_remove_client.to_string(),
            id.clone(),
            account_type.clone(),
            subbranch.clone(),
        )
        .await?;
    }

    Ok(())
}
