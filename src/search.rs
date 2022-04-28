use super::preludes::diesel_prelude::*;
use super::preludes::rocket_prelude::*;
use super::BMDBConn;

#[derive(Debug, PartialEq, FromFormField)]
pub enum SearchOption {
    ClientName,
    AccountID,
}

#[derive(Debug, FromForm, Serialize)]
pub struct SearchResultView {
    result_name: String,
    result_desc: String,
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

macro_rules! get_search_result {
    ($searchOption: expr; $struct_name: ident; $table_name: ident;$search: expr; $conn: expr; $($attr: ident),+) => {
        match (&$searchOption, &$conn, &$search){
            (searchOption, conn, search_ref)=>{
                let mut filter_results: Vec<$struct_name> = Vec::new();
                $(
                    if(searchOption.contains(&concat!(stringify!($struct_name), ".", stringify!($attr)).to_string())){
                        let search_copy = search_ref.clone();
                        filter_results.extend(
                            conn
                                .run(move |conn| {
                                    $table_name::dsl::$table_name
                                        .filter($table_name::dsl::$attr.like(format!("%{0}%", search_copy)))
                                        .limit(5)
                                        .load::<$struct_name>(conn)
                                        .expect("Error loading clients")
                                })
                                .await.into_iter()
                        )
                    }
                )*
        filter_results}}
    };
}

#[get("/search?<search>&<searchOption>")]
pub async fn search(conn: BMDBConn, search: String, searchOption: Vec<String>) -> Template {
    // let filter_result = conn
    //     .run(move |conn| {
    //         client::dsl::client
    //             .filter(client::dsl::clientName.like(format!("%{search}%")))
    //             .limit(5)
    //             .load::<Client>(conn)
    //             .expect("Error loading clients")
    //     })
    //     .await;

    let filter_results = get_search_result!(searchOption;Client;client; search; conn; clientID,clientName,clientAddr);

    eprintln!(
        "searchOptions: {searchOption:?}, Query {search} invoked, {num} entries found",
        num = filter_results.len()
    );

    let mut result_view = ResultContext {
        search: search.clone(),
        ..<ResultContext as Default>::default()
    };

    for client in &filter_results {
        result_view.results.push(SearchResultView {
            result_name: client.clientID.to_string(),
            result_desc: client
                .clientName
                .as_ref()
                .unwrap_or(&"No name".to_string())
                .to_string(),
        });
    }
    Template::render("results", &result_view)
}
