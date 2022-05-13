// table! {
//     account (accountID) {
//         accountID -> Char,
//         balance -> Nullable<Decimal>,
//         openDate -> Nullable<Date>,
//     }
// }

// table! {
//     accountmanagement (subbranchName, clientID) {
//         subbranchName -> Varchar,
//         clientID -> Char,
//         savingAccountID -> Nullable<Char>,
//         checkingAccountID -> Nullable<Char>,
//     }
// }

// table! {
//     checkingaccount (accountID) {
//         accountID -> Char,
//         balance -> Nullable<Decimal>,
//         openDate -> Nullable<Date>,
//         overdraft -> Nullable<Decimal>,
//     }
// }

// table! {
//     client (clientID) {
//         clientID -> Char,
//         employeeID -> Nullable<Char>,
//         clientName -> Nullable<Varchar>,
//         clientTel -> Nullable<Varchar>,
//         clientAddr -> Nullable<Varchar>,
//         contactName -> Nullable<Varchar>,
//         contactTel -> Nullable<Varchar>,
//         contactEmail -> Nullable<Varchar>,
//         contactRelationship -> Nullable<Varchar>,
//         serviceType -> Nullable<Varchar>,
//     }
// }

// table! {
//     department (departmentID) {
//         departmentID -> Char,
//         departmentName -> Nullable<Varchar>,
//         departmentType -> Nullable<Varchar>,
//     }
// }

// table! {
//     employee (employeeID) {
//         employeeID -> Char,
//         subbranchName -> Nullable<Varchar>,
//         departmentID -> Nullable<Char>,
//         employeeTel -> Nullable<Varchar>,
//         employeeAddr -> Nullable<Varchar>,
//         employmentCommenceDate -> Nullable<Date>,
//     }
// }

// table! {
//     loan (loanID) {
//         loanID -> Char,
//         subbranchName -> Varchar,
//     }
// }

// table! {
//     manager (employeeID) {
//         employeeID -> Char,
//         subbranchName -> Nullable<Varchar>,
//         departmentID -> Nullable<Char>,
//         employeeTel -> Nullable<Varchar>,
//         employeeAddr -> Nullable<Varchar>,
//         employmentCommenceDate -> Nullable<Date>,
//     }
// }

// table! {
//     own (accountID, clientID) {
//         accountID -> Char,
//         clientID -> Char,
//         lastVisitTime -> Nullable<Datetime>,
//     }
// }

// table! {
//     payment (loanID) {
//         loanID -> Char,
//         amount -> Nullable<Decimal>,
//     }
// }

// table! {
//     receiveloan (loanID, clientID) {
//         loanID -> Char,
//         clientID -> Char,
//     }
// }

// table! {
//     savingaccount (accountID) {
//         accountID -> Char,
//         balance -> Nullable<Decimal>,
//         openDate -> Nullable<Date>,
//         interest -> Nullable<Float>,
//         currencyType -> Nullable<Varchar>,
//     }
// }

// table! {
//     subbranch (subbranchName) {
//         subbranchName -> Varchar,
//         city -> Nullable<Varchar>,
//         subbranchAsset -> Nullable<Decimal>,
//     }
// }

// joinable!(accountmanagement -> client (clientID));
// joinable!(accountmanagement -> subbranch (subbranchName));
// joinable!(checkingaccount -> account (accountID));
// joinable!(client -> employee (employeeID));
// joinable!(employee -> department (departmentID));
// joinable!(employee -> subbranch (subbranchName));
// joinable!(loan -> subbranch (subbranchName));
// joinable!(manager -> employee (employeeID));
// joinable!(own -> account (accountID));
// joinable!(own -> client (clientID));
// joinable!(payment -> loan (loanID));
// joinable!(receiveloan -> client (clientID));
// joinable!(receiveloan -> loan (loanID));
// joinable!(savingaccount -> account (accountID));

// allow_tables_to_appear_in_same_query!(
//     account,
//     accountmanagement,
//     checkingaccount,
//     client,
//     department,
//     employee,
//     loan,
//     manager,
//     own,
//     payment,
//     receiveloan,
//     savingaccount,
//     subbranch,
// );
