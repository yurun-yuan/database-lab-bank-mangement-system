use super::preludes::rocket_prelude::*;
use crate::{error_template, unwrap_or_return, utility::GenericError};
use bigdecimal::Zero;
use sqlx::types::BigDecimal;

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

pub enum LoanStatus {
    Unpaid,
    BeingPaid,
    Paid,
}

impl LoanStatus {
    pub fn new(pay_amount: &BigDecimal, loan_amount: &BigDecimal) -> Result<Self, String> {
        if pay_amount.is_zero() && !loan_amount.is_zero() {
            Ok(Self::Unpaid)
        } else if pay_amount < loan_amount {
            Ok(Self::BeingPaid)
        } else if pay_amount == loan_amount {
            Ok(Self::Paid)
        } else {
            Err(format!(
                "Loaned amount is {} while paid amount is {}",
                loan_amount, pay_amount
            ))
        }
    }
}

impl std::fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unpaid => "Unpaid",
                Self::BeingPaid => "Being paid",
                Self::Paid => "Paid",
            }
        )
    }
}

#[get("/profile/loan?<id>")]
pub async fn loan_profile(mut db: Connection<BankManage>, id: String) -> Template {
    let (loan, associated_clients, associated_payments) =
        unwrap_or_return!(query_loan(&mut db, id.clone()).await, "Error querying loan");
    let pay_amount: sqlx::types::BigDecimal =
        associated_payments.iter().map(|pay| &pay.amount).sum();
    let status = unwrap_or_return!(
        LoanStatus::new(&pay_amount, &loan.amount),
        "Unexpected: Aggregated payment is more than the loaded amount"
    );
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
pub async fn query_loan(
    db: &mut Connection<BankManage>,
    id: String,
) -> Result<(Loan, Vec<ReceiveLoan>, Vec<Payment>), GenericError> {
    let loan = sqlx::query_as!(Loan, "SELECT * FROM loan WHERE loanID=?", id)
        .fetch_one(&mut **db)
        .await?;
    let associated_clients =
        sqlx::query_as!(ReceiveLoan, "SELECT * FROM receiveLoan WHERE loanID=?", id)
            .fetch_all(&mut **db)
            .await?;
    let associated_payments = sqlx::query_as!(Payment, "SELECT * FROM payment WHERE loanID=? ORDER BY date", id)
        .fetch_all(&mut **db)
        .await?;
    Ok((loan, associated_clients, associated_payments))
}
