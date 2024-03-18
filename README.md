# rust_db_rusqlite
A simple Rust project that demonstrates how to create and interact with an SQLite database using the "rusqlite" crate.
Rusqlite is an ergonomic wrapper for using SQLite from Rust.

## Installation
1. Clone this repository:
```
git clone https://github.com/qxf2/rust_db_rusqlite.git
```
2. Change to project directory:
```
cd rust_db_rusqlite/rust_db_rusqlite
```

3. Build the project : 
```
cargo build
```
## Usage
Run the program to create the SQLite database and interact with it
```
cargo run
```
The program will create an SQLite database file named employee.db in the project directory, if the database doesn't exist and open the database , if the database exist ! 
It will insert a sample data into the database tables created. 
