use super::preludes::rocket_prelude::*;
use crate::{commit, error_template, rollback, start_transaction, unwrap_or, unwrap_or_return};
use sqlx::Executor;

#[get("/delete/loan?<id>")]
pub async fn delete_payment(mut db: Connection<BankManage>, id: String) -> Template {
    let (loan, _, associated_payments) = unwrap_or_return!(
        super::loan_profile::query_loan(&mut db, &id).await,
        "Error querying loan"
    );
    let pay_amount: sqlx::types::BigDecimal =
        associated_payments.iter().map(|pay| &pay.amount).sum();
    let status = unwrap_or_return!(
        super::loan_profile::LoanStatus::new(&pay_amount, &loan.amount),
        "Unexpected: Aggregated payment is more than the loaded amount"
    );
    if matches!(status, super::loan_profile::LoanStatus::BeingPaid) {
        return error_template!("The loan being paid, deletion is forbidden");
    }

    start_transaction!(db);

    unwrap_or!(
        sqlx::query("DELETE FROM payment WHERE loanID=?")
            .bind(&id)
            .execute(&mut *db)
            .await,
        e,
        {
            rollback!(db);
            return error_template!(e, "Error removing associated payment");
        }
    );

    unwrap_or!(
        sqlx::query("DELETE FROM receiveloan WHERE loanID=?")
            .bind(&id)
            .execute(&mut *db)
            .await,
        e,
        {
            rollback!(db);
            return error_template!(e, "Error removing receiveloan relation");
        }
    );

    unwrap_or!(
        sqlx::query("DELETE FROM loan WHERE loanID=?")
            .bind(&id)
            .execute(&mut *db)
            .await,
        e,
        {
            rollback!(db);
            return error_template!(e, "Error removing loan attribute");
        }
    );

    commit!(db);

    Template::render("delete-payment-success", &Context::default())
}
