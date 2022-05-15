CREATE TABLE `account` (
  `accountID` char(64) NOT NULL,
  `balance` decimal(64, 2) DEFAULT NULL,
  `openDate` date DEFAULT NULL,
  PRIMARY KEY (`accountID`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `subbranch` (
  `subbranchName` varchar(64) NOT NULL,
  `city` varchar(64) DEFAULT NULL,
  `subbranchAsset` decimal(64, 2) DEFAULT NULL,
  PRIMARY KEY (`subbranchName`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `savingaccount` (
  `accountID` char(64) NOT NULL,
  `balance` decimal(64, 2) DEFAULT NULL,
  `openDate` date DEFAULT NULL,
  `interest` float DEFAULT NULL,
  `currencyType` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`accountID`),
  CONSTRAINT `FK_accounttype_saving` FOREIGN KEY (`accountID`) REFERENCES `account` (`accountID`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `loan` (
  `loanID` char(64) NOT NULL,
  `amount` decimal(64, 2) NOT NULL,
  `subbranchName` varchar(64) NOT NULL,
  PRIMARY KEY (`loanID`),
  KEY `FK_paidby` (`subbranchName`),
  CONSTRAINT `FK_paidby` FOREIGN KEY (`subbranchName`) REFERENCES `subbranch` (`subbranchName`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `payment` (
  `loanID` char(64) NOT NULL,
  `date` date NOT NULL,
  `amount` decimal(64, 2) NOT NULL,
  PRIMARY KEY (`loanID`),
  CONSTRAINT `FK_pay` FOREIGN KEY (`loanID`) REFERENCES `loan` (`loanID`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `department` (
  `departmentID` char(64) NOT NULL,
  `departmentName` varchar(64) DEFAULT NULL,
  `departmentType` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`departmentID`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `employee` (
  `employeeID` char(64) NOT NULL,
  `subbranchName` varchar(64) DEFAULT NULL,
  `departmentID` char(64) DEFAULT NULL,
  `employeeTel` varchar(64) DEFAULT NULL,
  `employeeAddr` varchar(64) DEFAULT NULL,
  `employmentCommenceDate` date DEFAULT NULL,
  PRIMARY KEY (`employeeID`),
  KEY `FK_in` (`departmentID`),
  KEY `FK_work_at` (`subbranchName`),
  CONSTRAINT `FK_in` FOREIGN KEY (`departmentID`) REFERENCES `department` (`departmentID`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  CONSTRAINT `FK_work_at` FOREIGN KEY (`subbranchName`) REFERENCES `subbranch` (`subbranchName`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `manager` (
  `employeeID` char(64) NOT NULL,
  `subbranchName` varchar(64) DEFAULT NULL,
  `departmentID` char(64) DEFAULT NULL,
  `employeeTel` varchar(64) DEFAULT NULL,
  `employeeAddr` varchar(64) DEFAULT NULL,
  `employmentCommenceDate` date DEFAULT NULL,
  PRIMARY KEY (`employeeID`),
  CONSTRAINT `FK_manager` FOREIGN KEY (`employeeID`) REFERENCES `employee` (`employeeID`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `client` (
  `clientID` char(64) NOT NULL,
  `employeeID` char(64) DEFAULT NULL,
  `clientName` varchar(64) DEFAULT NULL,
  `clientTel` varchar(64) DEFAULT NULL,
  `clientAddr` varchar(64) DEFAULT NULL,
  `contactName` varchar(64) DEFAULT NULL,
  `contactTel` varchar(64) DEFAULT NULL,
  `contactEmail` varchar(64) DEFAULT NULL,
  `contactRelationship` varchar(64) DEFAULT NULL,
  `serviceType` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`clientID`),
  KEY `FK_servicetype` (`employeeID`),
  CONSTRAINT `FK_servicetype` FOREIGN KEY (`employeeID`) REFERENCES `employee` (`employeeID`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `receiveloan` (
  `loanID` char(64) NOT NULL,
  `clientID` char(64) NOT NULL,
  PRIMARY KEY (`loanID`, `clientID`),
  KEY `FK_receive_client` (`clientID`),
  CONSTRAINT `FK_receive_client` FOREIGN KEY (`clientID`) REFERENCES `client` (`clientID`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  CONSTRAINT `FK_receive_loan` FOREIGN KEY (`loanID`) REFERENCES `loan` (`loanID`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `own` (
  `accountID` char(64) NOT NULL,
  `clientID` char(64) NOT NULL,
  `lastVisitTime` datetime DEFAULT NULL,
  PRIMARY KEY (`accountID`, `clientID`),
  KEY `FK_own_client` (`clientID`),
  CONSTRAINT `FK_own_account` FOREIGN KEY (`accountID`) REFERENCES `account` (`accountID`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  CONSTRAINT `FK_own_client` FOREIGN KEY (`clientID`) REFERENCES `client` (`clientID`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `checkingaccount` (
  `accountID` char(64) NOT NULL,
  `balance` decimal(64, 2) DEFAULT NULL,
  `openDate` date DEFAULT NULL,
  `overdraft` decimal(64, 2) DEFAULT NULL,
  PRIMARY KEY (`accountID`),
  CONSTRAINT `FK_accountype_checking` FOREIGN KEY (`accountID`) REFERENCES `account` (`accountID`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `accountmanagement` (
  `subbranchName` varchar(64) NOT NULL,
  `clientID` char(64) NOT NULL,
  `savingAccountID` char(64) DEFAULT NULL,
  `checkingAccountID` char(64) DEFAULT NULL,
  PRIMARY KEY (`subbranchName`, `clientID`),
  KEY `FK_accountmanagement_client` (`clientID`),
  CONSTRAINT `FK_accountmanagement_client` FOREIGN KEY (`clientID`) REFERENCES `client` (`clientID`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  CONSTRAINT `FK_accountmanagement_subbranch` FOREIGN KEY (`subbranchName`) REFERENCES `subbranch` (`subbranchName`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
