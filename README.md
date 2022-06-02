# Bank Management System

This project realizes a bank management system that meets the requirement specified in  [requirement.pdf](doc\requirement.pdf). The frontend is written with [Handlebars](https://handlebarsjs.com/), the database used is MySQL, the backend is written with [Rust Programming Language](https://www.rust-lang.org/) and the implementation is heavily dependent on the web framework [Rocket](https://rocket.rs/). 

## Build & Run

To build the project, you need complete toolchain for rust. It is recommended to use `Cargo` for crate management. 

To build up the database needed by the program, run [up.sql](migrations\up.sql) on the backend server and the tables are created in the selected database. 

To specify the database, change `url` in  [Rocket.toml](Rocket.toml) to your own database. 

A simple `cargo run` or `cargo run -r` if you want to run a release version will deploy the website at the specified port. 

## Report

See [report](doc%5Creport.pdf). 