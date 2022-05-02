
drop table if exists saving_account;

drop table if exists employee;

drop table if exists client;

drop table if exists own;

drop table if exists receiveLoan;

drop table if exists payment;

drop table if exists checkingAccount;

drop table if exists subbranch;

drop table if exists manager;

drop table if exists account;

drop table if exists accountManagement;

drop table if exists loan;

drop table if exists department;

/*==============================================================*/
/* Table: saving_account                                                  */
/*==============================================================*/
create table saving_account
(
   accountID                 char(64) not null,
   balance                   decimal(2,2),
   openDate                 date,
   interest                   float(8),
   currencyType                 varchar(64),
   primary key (accountID)
);

/*==============================================================*/
/* Table: employee                                                    */
/*==============================================================*/
create table employee
(
   employeeID                 char(64) not null,
   subbranchName                 varchar(64),
   departmentID                  char(64),
   employeeTel                 numeric(64,0),
   employeeAddr               varchar(64),
   employmentCommenceDate               date,
   primary key (employeeID)
);

/*==============================================================*/
/* Table: client                                                 */
/*==============================================================*/
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

/*==============================================================*/
/* Table: own                                                    */
/*==============================================================*/
create table own
(
   accountID                 char(64) not null,
   clientID                 char(64) not null,
   lastVisitTime               datetime,
   primary key (accountID, clientID)
);

/*==============================================================*/
/* Table: receiveLoan                                                    */
/*==============================================================*/
create table receiveLoan
(
   loanID                  char(64) not null,
   clientID                 char(64) not null,
   primary key (loanID, clientID)
);

/*==============================================================*/
/* Table: payment                                                    */
/*==============================================================*/
create table payment
(
   loanID                  char(64) not null,
   amount                   decimal(2,2),
   primary key (loanID)
);

/*==============================================================*/
/* Table: checkingAccount                                                  */
/*==============================================================*/
create table checkingAccount
(
   accountID                 char(64) not null,
   balance                   decimal(2,2),
   openDate                 date,
   overdraft                  decimal(2,2),
   primary key (accountID)
);

/*==============================================================*/
/* Table: subbranch                                                    */
/*==============================================================*/
create table subbranch
(
   subbranchName                 varchar(64) not null,
   city                 varchar(64),
   subbranchAsset                 decimal(2,2),
   primary key (subbranchName)
);

/*==============================================================*/
/* Table: manager                                                    */
/*==============================================================*/
create table manager
(
   employeeID                 char(64) not null,
   subbranchName                 varchar(64),
   departmentID                  char(64),
   employeeTel                 numeric(64,0),
   employeeAddr               varchar(64),
   employmentCommenceDate               date,
   primary key (employeeID)
);

/*==============================================================*/
/* Table: account                                                    */
/*==============================================================*/
create table account
(
   accountID                 char(64) not null,
   balance                   decimal(2,2),
   openDate                 date,
   primary key (accountID)
);

/*==============================================================*/
/* Table: accountManagement                                                  */
/*==============================================================*/
create table accountManagement
(
   subbranchName                 varchar(64) not null,
   clientID                 char(64) not null,
   savingAccountID               char(64),
   checkingAccountID               char(64),
   primary key (subbranchName, clientID)
);

/*==============================================================*/
/* Table: loan                                                    */
/*==============================================================*/
create table loan
(
   loanID                  char(64) not null,
   subbranchName                 varchar(64) not null,
   primary key (loanID)
);

/*==============================================================*/
/* Table: department                                                    */
/*==============================================================*/
create table department
(
   departmentID                  char(64) not null,
   departmentName                 varchar(64),
   departmentType                 varchar(64),
   primary key (departmentID)
);

alter table saving_account add constraint "FK_accounttype-saving" foreign key (accountID)
      references account (accountID) on delete restrict on update restrict;

alter table employee add constraint FK_in foreign key (departmentID)
      references department (departmentID) on delete restrict on update restrict;

alter table employee add constraint FK_work_at foreign key (subbranchName)
      references subbranch (subbranchName) on delete restrict on update restrict;

alter table client add constraint FK_servicetype foreign key (employeeID)
      references employee (employeeID) on delete restrict on update restrict;

alter table own add constraint FK_own_account foreign key (accountID)
      references account (accountID) on delete restrict on update restrict;

alter table own add constraint FK_own_client foreign key (clientID)
      references client (clientID) on delete restrict on update restrict;

alter table receiveLoan add constraint FK_receive_loan foreign key (loanID)
      references loan (loanID) on delete restrict on update restrict;

alter table receiveLoan add constraint FK_receive_client foreign key (clientID)
      references client (clientID) on delete restrict on update restrict;

alter table payment add constraint FK_pay foreign key (loanID)
      references loan (loanID) on delete restrict on update restrict;

alter table checkingAccount add constraint "FK_accountype-checking" foreign key (accountID)
      references account (accountID) on delete restrict on update restrict;

alter table manager add constraint "FK_manager" foreign key (employeeID)
      references employee (employeeID) on delete restrict on update restrict;

alter table accountManagement add constraint FK_accountmanagement-subbranch foreign key (subbranchName)
      references subbranch (subbranchName) on delete restrict on update restrict;

alter table accountManagement add constraint FK_accountmanagement-client foreign key (clientID)
      references client (clientID) on delete restrict on update restrict;

alter table loan add constraint FK_paidby foreign key (subbranchName)
      references subbranch (subbranchName) on delete restrict on update restrict;

