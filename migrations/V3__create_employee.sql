create table employee (
    id int PRIMARY KEY AUTO INCREMENTS,
    first_name varchar(25) not null,
    last_name varchar(25) not null,
    birth_date DATE,
    hire_date DATE,
    gander ENUM('M', 'F')
)