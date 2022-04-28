create table client
(
   clientID                 char(64) not null,
   employeeID                 char(64),
   clientName                 varchar(64),
   clientTel                   varchar(64),
   clientAddr                 varchar(64),
   contactName                varchar(64),
   contanceTel                   varchar(64),
   contactEmail                varchar(64),
   contactRelationship                   varchar(64),
   serviceType                 varchar(64),
   primary key (ClientID)
);