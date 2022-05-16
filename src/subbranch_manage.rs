use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::AddAssign;

use chrono::Datelike;
use sqlx::types::BigDecimal;

use super::preludes::rocket_prelude::*;
use crate::unwrap_or_return;
use crate::utility::GenericError;
use sqlx::types::chrono::NaiveDate;

const DISPLAY_DURATION_YEAR: usize = 5;
type DisplayedStatistic = [(String, [(String, [(String, String); 3]); 4]); DISPLAY_DURATION_YEAR];

#[derive(Serialize)]
struct SubbranchProfileContext {
    subbranch_name: String,
    subbranch_city: String,
    subbranch_asset: String,
    statistics: HashMap<String, DisplayedStatistic>,
}

#[get("/profile/subbranch?<name>")]
pub async fn subbranch_profile(mut db: Connection<BankManage>, name: &str) -> Template {
    // subbranch info
    let subbranch = unwrap_or_return!(
        query_subbranch(&mut db, name).await,
        "Error querying subbranch"
    );

    // associated accounts
    let mut accounts: [HashSet<Account>; 2] = Default::default();
    for (i, specific_account_ids) in unwrap_or_return!(
        query_associated_account(&mut db, name).await,
        "Error querying account"
    )
    .into_iter()
    .enumerate()
    {
        let mut specific_accounts = HashSet::new();
        for specific_account_id in specific_account_ids {
            specific_accounts.insert(Account::from(
                unwrap_or_return!(
                    crate::account_manage::query::query_account_by_id(
                        &mut db,
                        &specific_account_id
                    )
                    .await,
                    "Error querying account details"
                )
                .0,
            ));
        }
        accounts[i] = specific_accounts;
    }

    //associated loans
    let loans = unwrap_or_return!(
        query_associated_loans(&mut db, name).await,
        "Error querying loans"
    );

    let mut loans_and_payment = HashSet::<(Loan, Vec<Payment>)>::new();
    for loan in loans {
        let (_, _, payments) = unwrap_or_return!(
            crate::loan_profile::query_loan(&mut db, &loan.loanID).await,
            "Error querying payments"
        );
        loans_and_payment.insert((loan, payments));
    }

    let (cur_year, _) = (
        chrono::Local::today().year(),
        chrono::Local::today().month0(),
    );
    let start_date = NaiveDate::from_yo(cur_year - DISPLAY_DURATION_YEAR as i32 + 1, 1);

    type Statistic<T> = [[[T; 3]; 4]; DISPLAY_DURATION_YEAR];

    fn get_statistics<InputIter, T>(input_iter: InputIter, start_date: &NaiveDate) -> Statistic<T>
    where
        T: AddAssign + Default,
        InputIter: Iterator<Item = (T, NaiveDate)>,
    {
        let mut result = Statistic::<T>::default();
        for (item, date) in input_iter.filter(|(_, date)| date >= start_date) {
            let (year, season, offset) = (date.year(), date.month0() / 3u32, date.month0() % 3);
            result[(year - start_date.year()) as usize][season as usize][offset as usize] += item;
        }
        result
    }

    let mut account_statistic: [Statistic<BigDecimal>; 2] = Default::default();

    for i in 0..2 {
        account_statistic[i] = get_statistics(
            accounts[i]
                .iter()
                .map(|item| (item.balance.clone(), item.openDate)),
            &start_date,
        );
    }

    let payment_statistic: Statistic<BigDecimal> = get_statistics(
        loans_and_payment
            .iter()
            .map(|(_, payments)| {
                payments
                    .iter()
                    .map(|payment| (payment.amount.clone(), payment.date))
            })
            .flatten(),
        &start_date,
    );

    fn statistic_context<T: Display>(
        statistic: &Statistic<T>,
        start_year: i32,
    ) -> DisplayedStatistic {
        let mut result: DisplayedStatistic = Default::default();
        let season_label = ["Q1", "Q2", "Q3", "Q4"];
        let month_label = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sept", "Oct", "Nov", "Dec",
        ];
        for i in 0..statistic.len() {
            result[i].0 = (start_year as usize + i).to_string();
            for j in 0..statistic[i].len() {
                result[i].1[j].0 = season_label[j].to_string();
                for k in 0..statistic[i][j].len() {
                    result[i].1[j].1[k].0 = month_label[j * 3 + k].to_string();
                    result[i].1[j].1[k].1 = statistic[i][j][k].to_string();
                }
            }
        }
        result.reverse();
        result
    }

    Template::render(
        "subbranch-profile",
        &SubbranchProfileContext {
            subbranch_name: subbranch.subbranchName,
            subbranch_city: subbranch.city,
            subbranch_asset: subbranch.subbranchAsset.to_string(),
            statistics: HashMap::from([
                (
                    "saving_account".to_string(),
                    statistic_context(&account_statistic[0], start_date.year()),
                ),
                (
                    "checking_account".to_string(),
                    statistic_context(&account_statistic[1], start_date.year()),
                ),
                (
                    "payment".to_string(),
                    statistic_context(&payment_statistic, start_date.year()),
                ),
            ]),
        },
    )
}

pub async fn set_subbranch_asset(
    db: &mut Connection<BankManage>,
    subbranch: &str,
    new_asset: &BigDecimal,
) -> Result<(), GenericError> {
    sqlx::query("UPDATE subbranch SET subbranchAsset=? WHERE subbranchName=?")
        .bind(new_asset.to_string())
        .bind(subbranch)
        .execute(&mut **db)
        .await?;
    Ok(())
}

pub async fn query_subbranch(
    db: &mut Connection<BankManage>,
    subbranch: &str,
) -> Result<Subbranch, GenericError> {
    Ok(sqlx::query_as!(
        Subbranch,
        "SELECT * FROM subbranch WHERE subbranchName=?",
        subbranch
    )
    .fetch_one(&mut **db)
    .await?)
}

/// Returns [SavingAccountIDs, CheckingAccountIDs]
async fn query_associated_account(
    db: &mut Connection<BankManage>,
    subbranch: &str,
) -> Result<[HashSet<String>; 2], GenericError> {
    let account_manage = sqlx::query_as!(
        AccountManagement,
        "SELECT * FROM accountmanagement WHERE subbranchName=?",
        subbranch
    )
    .fetch_all(&mut **db)
    .await?;
    let mut account_ids = [HashSet::<String>::new(), HashSet::<String>::new()];
    account_manage.into_iter().for_each(|account_manage| {
        if let Some(id) = account_manage.savingAccountID {
            account_ids[0].insert(id);
        }
        if let Some(id) = account_manage.checkingAccountID {
            account_ids[1].insert(id);
        }
    });
    Ok(account_ids)
}

async fn query_associated_loans(
    db: &mut Connection<BankManage>,
    subbranch: &str,
) -> Result<HashSet<Loan>, GenericError> {
    Ok(
        sqlx::query_as!(Loan, "SELECT * FROM loan WHERE subbranchName=?", subbranch)
            .fetch_all(&mut **db)
            .await?
            .into_iter()
            .collect(),
    )
}
