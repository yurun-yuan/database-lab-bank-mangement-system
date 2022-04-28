table! {
    client (clientID) {
        clientID -> Char,
        employeeID -> Nullable<Char>,
        clientName -> Nullable<Varchar>,
        clientTel -> Nullable<Varchar>,
        clientAddr -> Nullable<Varchar>,
        contactName -> Nullable<Varchar>,
        contanceTel -> Nullable<Varchar>,
        contactEmail -> Nullable<Varchar>,
        contactRelationship -> Nullable<Varchar>,
        serviceType -> Nullable<Varchar>,
    }
}
