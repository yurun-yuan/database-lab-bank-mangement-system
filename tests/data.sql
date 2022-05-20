INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('Hefei Subbranch', 'Hefei', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('Beijing Subbranch', 'Beijing', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('Shanghai Subbranch', 'Shanghai', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('Shenzhen Subbranch', 'Shenzhen', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('Guangzhou Subbranch', 'Guangzhou', 100000.00);
INSERT INTO `subbranch` (`subbranchName`, `city`, `subbranchAsset`)
VALUES ('Nanjing Subbranch', 'Nanjing', 100000.00);
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
        'Hefei Subbranch',
        'department0ID',
        '0000',
        'Address for employee0',
        DATE(NOW())
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
        'employee1ID',
        'Beijing Subbranch',
        'department0ID',
        '0001',
        'Address for employee1',
        DATE(NOW())
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
        'employee2ID',
        'Shanghai Subbranch',
        'department0ID',
        '0002',
        'Address for employee2',
        DATE(NOW())
    );