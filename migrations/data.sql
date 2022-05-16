INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('subbranch0', 'Hefei', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('subbranch1', 'Beijing', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('subbranch2', 'Shanghai', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('subbranch3', 'Shenzhen', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('subbranch4', 'Guangzhou', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('subbranch5', 'Nanjing', 100000.00);
INSERT INTO `department` (
        `departmentID`,
        `departmentName`,
        `departmentType`
    )
VALUES(
        'department0ID',
        'department0',
        'departmentType0'
    );
INSERT INTO `employee` (
        `employeeID`,
        `subbranchName`,
        `departmentID`,
        `employeeTel`,
        `employeeAddr`,
        `employmentCommenceDate`
    )
VALUES(
        'employee0ID',
        'subbranch0',
        'department0ID',
        '1234567890',
        'Address for employee0',
         DATE(NOW())
    );