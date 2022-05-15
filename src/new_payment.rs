use std::collections::HashMap;

use super::preludes::rocket_prelude::*;
use crate::subbranch_manage::*;
use crate::{commit, error_template, rollback, start_transaction, unwrap_or, unwrap_or_return};
use bigdecimal::Zero;
use chrono::Local;
use sqlx::Executor;

#[get("/new/payment?<id>")]
pub async fn get_new_loan(id: String) -> Template {
    Template::render("new-payment", &HashMap::from([("id", id)]))
}

#[derive(Debug, FromForm, Default, Serialize)]
pub struct Submit {
    amount: String,
}

#[post("/new/payment?<id>", data = "<form>")]
pub async fn submit(
    mut db: Connection<BankManage>,
    id: String,
    form: Form<Contextual<'_, Submit>>,
) -> Template {
    let value = match form.value {
        Some(ref value) => value,
        None => return error_template!("Error adding payment: failed to receive form"),
    };
    let (loan, _, associated_payments) = unwrap_or_return!(
        super::loan_profile::query_loan(&mut db, id.clone()).await,
        "Error querying loan"
    );
    let pay_amount: sqlx::types::BigDecimal =
        associated_payments.iter().map(|pay| &pay.amount).sum();
    let status = unwrap_or_return!(
        super::loan_profile::LoanStatus::new(&pay_amount, &loan.amount),
        "Unexpected: Aggregated payment is more than the loaded amount"
    );
    if matches!(status, super::loan_profile::LoanStatus::Paid) {
        return error_template!("The loan is already paid, no more payment is received");
    }
    let new_payment: sqlx::types::BigDecimal = unwrap_or_return!(
        value.amount.parse(),
        "Invalid input: cannot parse input amount into decimal"
    );

    // Rule 1: the payment should not outnumber the loaned money
    if &new_payment + &pay_amount > loan.amount {
        return error_template!(format!("The payment outnumbers the loaned amount. New payment is {new_payment}, previous payment is {pay_amount}, the loaned amount is {loan_amount}", loan_amount=loan.amount));
    }

    // Rule 2: the payment should not outnumber the assets of the subbranch
    let subbranch = unwrap_or_return!(
        query_subbranch(&mut db, &loan.subbranchName).await,
        "Fail to fetch information of the subbranch"
    );
    if new_payment > subbranch.subbranchAsset {
        return error_template!("The payment outnumber the assets of the subbranch");
    }

    // Rule 3: the payment should not be zero
    if new_payment.is_zero() {
        return error_template!("The payment should not be zero");
    }

    // Updating the database
    start_transaction!(db);
    let new_asset = &subbranch.subbranchAsset - &new_payment;
    unwrap_or!(
        set_subbranch_asset(&mut db, &subbranch.subbranchName, &new_asset).await,
        e,
        {
            rollback!(db);
            return error_template!(e, "Error updating the subbranch asset");
        }
    );
    unwrap_or!(
        sqlx::query("INSERT INTO payment (loanID, date, amount)VALUES(?, ?, ?)")
            .bind(&loan.loanID)
            .bind(Local::now().format("%Y-%m-%d").to_string())
            .bind(&new_payment)
            .execute(&mut *db)
            .await,
        e,
        {
            rollback!(db);
            return error_template!(e, "Error updating the payment record");
        }
    );
    commit!(db);
    Template::render("new-payment-success", &HashMap::from([("id", id)]))
}
