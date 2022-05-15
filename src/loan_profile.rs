use super::preludes::rocket_prelude::*;
use crate::{error_template, unwrap_or, utility::GenericError};
use bigdecimal::Zero;
use rocket::futures::TryStreamExt;

#[derive(Serialize)]
pub struct LoanProfileContext {
    pub loanID: String,
    pub subbranch: String,
    pub amount: String,
    pub associated_clients: Vec<String>,
    pub payments: Vec<PaymentProfileContext>,
    pub status: String,
    pub paid: String,
    pub unpaid: String,
}

#[derive(Serialize)]
pub struct PaymentProfileContext {
    pub date: String,
    pub amount: String,
}

macro_rules! unwrap_or_return {
    ($result:  expr, $info: literal) => {
        unwrap_or!($result, e, { return error_template!(e, $info) })
    };
}

#[get("/profile/loan?<id>")]
pub async fn client_profile(mut db: Connection<BankManage>, id: String) -> Template {
    let loan = unwrap_or_return!(
        sqlx::query_as!(Loan, "SELECT * FROM loan WHERE loanID=?", id)
            .fetch_one(&mut *db)
            .await,
        "Error querying loan"
    );
    let associated_clients = unwrap_or_return!(
        sqlx::query_as!(ReceiveLoan, "SELECT * FROM receiveLoan WHERE loanID=?", id)
            .fetch_all(&mut *db)
            .await,
        "Error querying loan"
    );
    let associated_payments = unwrap_or_return!(
        sqlx::query_as!(Payment, "SELECT * FROM payment WHERE loanID=?", id)
            .fetch_all(&mut *db)
            .await,
        "Error querying loan"
    );
    let pay_amount: sqlx::types::BigDecimal =
        associated_payments.iter().map(|pay| &pay.amount).sum();
    let status = if pay_amount.is_zero() {
        "Unpaid"
    } else if pay_amount < loan.amount {
        "Being paid"
    } else if pay_amount == loan.amount {
        "Paid"
    } else {
        return error_template!("Unexpected: Aggregated payment is more than the loaded amount");
    };
    let context = LoanProfileContext {
        loanID: id,
        subbranch: loan.subbranchName,
        amount: loan.amount.to_string(),
        associated_clients: associated_clients
            .into_iter()
            .map(|receive_loan| receive_loan.clientID)
            .collect(),
        payments: associated_payments
            .into_iter()
            .map(|payment| PaymentProfileContext {
                date: payment.date.to_string(),
                amount: payment.amount.to_string(),
            })
            .collect(),
        status: status.to_string(),
        paid: pay_amount.to_string(),
        unpaid: (loan.amount - pay_amount).to_string(),
    };
    Template::render("loan-profile", &context)
}
