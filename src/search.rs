use super::preludes::rocket_prelude::*;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

#[derive(Debug, FromForm, Serialize)]
pub struct SearchResultView {
    href: String,
    result_name: String,
    result_subtitle: String,
    result_desc: HashMap<String, String>,
}

#[derive(Debug, FromForm, Serialize, Default)]
pub struct ResultContext {
    search: String,
    results: Vec<SearchResultView>,
}

#[macro_export]
macro_rules! get_attr_list {
    ($struct_name: ident; $($attr: ident),+) => {
        vec![$(concat!(stringify!($struct_name),".", stringify!($attr)).to_string(),)*]
    };
}

trait Hightlight {
    fn highlight(&self, re: Regex, replace_with: &str) -> String;
    fn to_string(&self) -> String {
        self.highlight(Regex::new("").unwrap(), "")
    }
}

impl Hightlight for String {
    fn highlight(&self, re: Regex, replace_with: &str) -> String {
        if self.is_empty() {
            self.clone()
        } else {
            re.replace_all(self, replace_with).to_string()
        }
    }
}

impl Hightlight for Option<String> {
    fn highlight(&self, re: Regex, replace_with: &str) -> String {
        if self.is_none() || self.as_ref().unwrap().is_empty() {
            <String as Default>::default()
        } else {
            let str = self.as_ref().unwrap();
            re.replace_all(str, replace_with).to_string()
        }
    }
}

fn hightlight_string<Src: Hightlight>(search_ref: &'_ str, src: &Src) -> String {
    src.highlight(
        RegexBuilder::new(&format!("(?P<s>{0})", search_ref))
            .case_insensitive(true)
            .build()
            .expect("Regex pattern error during parsing search keys"),
        "<mark>$s</mark>",
    )
}

/// Returns `HashMap<PK, (struct of entity, HashMap<attr name, higlighted attr value>)>`
macro_rules! get_search_result {
    ($searchOption: expr; $struct_name: ident; $table_name: ident;$search: expr; $db: expr; $pk:ident, $($attr: ident),+) => {
        match (&$searchOption, &mut $db, &$search){
            (searchOption, db, search_ref)=>{
                let mut filter_results: HashMap<String, ($struct_name, HashMap<String, String>)> = HashMap::new();
                $(
                    if(searchOption.contains(&concat!(stringify!($struct_name), ".", stringify!($attr)).to_string())){
                        let query_statement = format!(concat!("SELECT * FROM ", stringify!($table_name), " WHERE ", stringify!($attr), " LIKE '%{0}%'"), search_ref);
                        let search_results=sqlx::query_as::<_, $struct_name>(&query_statement).fetch_all(&mut **db).await.unwrap_or(vec![]);
                        for search_result in search_results {
                            let new_value = (
                                stringify!($attr).to_string(),
                                hightlight_string(search_ref, &search_result.$attr)
                            );
                            match filter_results.entry(search_result.$pk.clone()){
                                std::collections::hash_map::Entry::Occupied(mut entry)=>{
                                    entry.get_mut().1.insert(new_value.0, new_value.1);}
                                std::collections::hash_map::Entry::Vacant(entry)=>{
                                    entry.insert((search_result, HashMap::from([new_value])));}
                            }
                        }
                    }
                )*
                filter_results
            }
        }
    };
}

#[get("/search?<search>&<searchOption>")]
pub async fn search(
    mut db: Connection<BankManage>,
    search: String,
    searchOption: Vec<String>,
) -> Template {
    // Search among clients
    let client_filter_results = get_search_result!(searchOption;Client;client; search; db;clientID, clientID,clientName,clientAddr,contactName);

    let mut result_view = ResultContext {
        search: search.clone(),
        ..<ResultContext as Default>::default()
    };

    for mut client in client_filter_results.into_values() {
        result_view.results.push(SearchResultView {
            href: "/profile/client?id=".to_string() + &client.0.clientID,
            result_subtitle: ToString::to_string(
                client.1.get("clientID").unwrap_or(&client.0.clientID),
            ),
            result_name: ToString::to_string(
                client
                    .1
                    .get("clientName")
                    .unwrap_or(&Hightlight::to_string(&client.0.clientName)),
            ),
            result_desc: if !search.is_empty() {
                client.1.remove("clientID");
                client.1.remove("clientName");
                client.1
            } else {
                <HashMap<String, String> as Default>::default()
            },
        });
    }

    // Search among accounts
    if searchOption.contains(&"Account.accountID".to_string()) {
        let account_results: Vec<Account> = sqlx::query_as(&format!(
            "SELECT * FROM account WHERE accountID LIKE '%{}%'",
            search
        ))
        .fetch_all(&mut *db)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Error querying account: {e_info}", e_info = e);
            vec![]
        });

        eprintln!("results of account: {account_results:?} for {search}",);
        for account_result in account_results {
            result_view.results.push(SearchResultView {
                href: "/profile/account?id=".to_string() + &account_result.accountID,
                result_name: "Account: ".to_string() + &account_result.accountID,
                result_subtitle: "".to_string(),
                result_desc: [
                    (
                        "Account ID".to_string(),
                        hightlight_string(&search, &account_result.accountID),
                    ),
                    ("Open Date".to_string(), account_result.openDate.to_string()),
                    ("Balance".to_string(), account_result.balance.to_string()),
                ]
                .into_iter()
                .collect(),
            });
        }
    }

    Template::render("results", &result_view)
}
