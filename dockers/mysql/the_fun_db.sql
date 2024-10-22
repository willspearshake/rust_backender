CREATE DATABASE `the_fun_db`;

CREATE USER 'the_cat' IDENTIFIED BY 'password';

GRANT ALL PRIVILEGES ON the_fun_db.* TO 'the_cat';

USE the_fun_db;

CREATE TABLE People (
    PersonID int,
    LastName varchar(255),
    FirstName varchar(255),
    Address varchar(255),
    City varchar(255)
);