use super::preludes::rocket_prelude::*;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

#[derive(Debug, FromForm, Serialize)]
pub struct SearchResultView {
    id: String,
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
        if self.len() == 0 {
            self.clone()
        } else {
            re.replace_all(self, replace_with).to_string()
        }
    }
}

impl Hightlight for Option<String> {
    fn highlight(&self, re: Regex, replace_with: &str) -> String {
        if self.is_none() || self.as_ref().unwrap().len() == 0 {
            <String as Default>::default()
        } else {
            let str = self.as_ref().unwrap();
            re.replace_all(&str, replace_with).to_string()
        }
    }
}

/// Returns `HashMap<PK, (struct of entity, HashMap<attr name, higlighted attr value>)>`
macro_rules! get_search_result {
    ($searchOption: expr; $struct_name: ident; $table_name: ident;$search: expr; $db: expr; $pk:ident, $($attr: ident),+) => {
        match (&$searchOption, &mut $db, &$search){
            (searchOption, db, search_ref)=>{
                let mut filter_results: HashMap<String, ($struct_name, HashMap<String, String>)> = HashMap::new();
                $(
                    if(searchOption.contains(&concat!(stringify!($struct_name), ".", stringify!($attr)).to_string())){
                        let search_copy = search_ref.clone();
                        let query_pattern=concat!("SELECT * FROM ", stringify!($table_name), " WHERE ", stringify!($attr), " LIKE '%{0}%'");
                        let query_statement=format!(concat!("SELECT * FROM ", stringify!($table_name), " WHERE ", stringify!($attr), " LIKE '%{0}%'"), search_copy);
                        let search_results=sqlx::query_as::<_, $struct_name>(&query_statement).fetch_all(&mut **db).await.unwrap_or(vec![]);
                        // let search_results = sqlx::query_as!($struct_name, concat!("SELECT * FROM ", stringify!($table_name), " WHERE ", stringify!($attr), " LIKE '%{?}%'"), search_copy).fetch_all(&mut **$db).await.unwrap_or(vec![]);
                        // db.run(move |db| {
                        //     $table_name::dsl::$table_name
                        //         .filter($table_name::dsl::$attr.like(format!("%{0}%", search_copy)))
                        //         .limit(64)
                        //         .load::<$struct_name>(db)
                        //         .expect("Error loading clients")
                        // }).await;
                        for search_result in search_results {
                            let new_value = (
                                stringify!($attr).to_string(),
                                search_result.$attr.highlight(
                                    RegexBuilder::new(&format!("(?P<s>{0})", search_ref))
                                        .case_insensitive(true)
                                        .build()
                                        .expect("Regex pattern error during parsing search keys"),
                                    "<mark>$s</mark>"
                                )
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
            id: client.0.clientID.clone(),
            result_subtitle: ToString::to_string(
                client.1.get("clientID").unwrap_or(&client.0.clientID),
            ),
            result_name: ToString::to_string(
                client
                    .1
                    .get("clientName")
                    .unwrap_or(&Hightlight::to_string(&client.0.clientName)),
            ),
            result_desc: if search.len() != 0 {
                client.1.remove("clientID");
                client.1.remove("clientName");
                client.1
            } else {
                <HashMap<String, String> as Default>::default()
            },
        });
    }

    // Search among accounts

    Template::render("results", &result_view)
}
