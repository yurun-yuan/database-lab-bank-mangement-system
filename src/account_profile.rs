use super::preludes::rocket_prelude::*;
use crate::account_manage::query::*;
use crate::error_template;

#[derive(Serialize)]
pub struct AccountProfileContext {
    pub accountID: String,
    pub balance: String,
    pub openDate: String,
    pub subbranch: String,
    pub associated_clients: Vec<String>,
    pub account_type: String,
    pub details: Vec<(String, String)>,
}

#[get("/profile/account?<id>")]
pub async fn account_profile(mut db: Connection<BankManage>, id: String) -> Template {
    let associated_clients = match query_associated_clients(&mut db, id.clone()).await {
        Ok(clients) => clients,
        Err(e) => return error_template!(e),
    };
    match query_account_by_id(&mut db, id.clone()).await {
        Ok((specific_account, subbranch)) => match specific_account {
            SpecificAccount::SavingAccount(saving_account) => Template::render(
                "account-profile",
                AccountProfileContext {
                    accountID: saving_account.accountID,
                    balance: saving_account.balance.unwrap().to_string(),
                    openDate: saving_account.openDate.unwrap().to_string(),
                    subbranch,
                    account_type: "saving account".to_string(),
                    details: vec![
                        (
                            "Interest".to_string(),
                            saving_account.interest.unwrap_or(0f32).to_string(),
                        ),
                        (
                            "Currency type".to_string(),
                            saving_account.currencyType.unwrap_or_else(|| "None".to_string()),
                        ),
                    ],
                    associated_clients,
                },
            ),
            SpecificAccount::CheckingAccount(checking_account) => Template::render(
                "account-profile",
                AccountProfileContext {
                    accountID: checking_account.accountID,
                    balance: checking_account.balance.unwrap().to_string(),
                    openDate: checking_account.openDate.unwrap().to_string(),
                    subbranch,
                    account_type: "checking account".to_string(),
                    details: vec![(
                        "overdraft".to_string(),
                        checking_account.overdraft.unwrap().to_string(),
                    )],
                    associated_clients,
                },
            ),
        },
        Err(e) => error_template!(e, "Error querying client"),
    }
}
