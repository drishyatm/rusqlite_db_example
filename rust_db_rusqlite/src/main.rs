use rusqlite::{Connection, Result};
use std::fs;


#[derive(Debug)]
struct Project {
    project_name: String,
    description: Option<String>,
    deadline: Option<String>,
}

#[derive(Debug)]
struct Task {
    id: i32,
    priority: i32,
    details: Option<String>,
    status: Option<String>,
    deadline: Option<String>,
    completed_on: Option<String>,
    project: String,
}

#[derive(Debug)]
struct Employee {
    id: i32,
    name: String,
    position: Option<String>,
}

fn create_tables(conn: &Connection) -> Result<()> {
    // Create the project table
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS project(
            project_name TEXT PRIMARY KEY,
            description  TEXT,
            deadline     DATE
        )",
        (),
    ) {
        Ok(_) => {
            println!("Created table project");            
        }
        Err(err) => return Err(err.into()),
    }

    // Create the task table
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            priority     INTEGER DEFAULT 1,
            details      TEXT,
            status       TEXT,
            deadline     DATE,
            completed_on DATE,
            project      TEXT NOT NULL REFERENCES project(project_name)
    
         )",
        (),
    ) {
        Ok(_) => {
            println!("Created table task");           
        }
        Err(err) => return Err(err.into()),
    }

    // Create the employee table
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS employees (
            id       INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            position TEXT
        )",
        (),
    ) {
        Ok(_) => {
            println!("Created table employee");           
        }
        Err(err) => return Err(err.into()),
    }
    Ok(())
}

fn db_exists(db_file_path: &str) -> bool {
    fs::metadata(db_file_path).is_ok()
}

fn insert_project(conn: &Connection, project: &Project) ->  Result<(), rusqlite::Error> {
    match conn.execute(
        "INSERT INTO project (project_name, description, deadline) VALUES (?1, ?2, ?3)",
        (&project.project_name, &project.description, &project.deadline),
    ) {
        Ok(_) => {println!("Inserting {:?} into the database.",project);
        Ok(())},
        Err(err) => Err(err),
    }
}

fn insert_task(conn: &Connection, task: &Task) ->  Result<(), rusqlite::Error> {
    match conn.execute(
        "INSERT INTO task (id, priority, details, status, deadline, completed_on, project) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &task.id,
            &task.priority,
            &task.details,
            &task.status,
            &task.deadline,
            &task.completed_on,
            &task.project,
        ),
    ) {
        Ok(_) => {println!("Inserting {:?} into the database.",task);
        Ok(())},
        Err(err) => Err(err),
    }
}

fn insert_employee(conn: &Connection, employee: &Employee) ->  Result<(), rusqlite::Error> {
    match conn.execute(
        "INSERT INTO employees (id, name, position) VALUES (?1, ?2, ?3)",
        (&employee.id, &employee.name, &employee.position),
    ) {
        Ok(_) => {println!("Inserting {:?} into the database.",employee);
        Ok(())},
        Err(err) => Err(err),
    }
}
fn main() -> Result<()>  {
    // File path for the employee database file
    let db_file_path = "employee.db";
    // check for DB Exists or not
    if !db_exists(&db_file_path) {         
        println!("No existing database! New database will be created in the path: {}", db_file_path);
    }
    else {        
        println!("Database Exists !!! Opened DB created at: {}", db_file_path);
    }
   
    // Connect to the database
    let conn = match Connection::open(db_file_path) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error connecting to database: {}.", e);
            return Err(e);
        }
    };
    match create_tables(&conn){
            Ok(()) => {
            println!("Tables created if they didn't exist.");
            // Initializing the project data 
            let project = Project {
                project_name: "Sample Project2".to_string(),
                description: Some("This is a sample project2".to_string()),
                deadline: Some("2023-12-31".to_string()),
            };
            // Insert new project into the database.
            match insert_project(&conn, &project) {
                Ok(_) => {
                        println!("Project data inserted successfully!");
                }                            
                Err(e) => {
                    println!("Failed to insert project: {:?}", e);                    
                    return Err(e.into());
                }
            }           
            // Initializing the task data 
            let task = Task {
                id: 3,
                priority: 2,
                details: Some("Complete task 1".to_string()),
                status: Some("Incomplete".to_string()),
                deadline: Some("2023-12-15".to_string()),
                completed_on: None,
                project: "Sample Project2".to_string(),
            };
            // Insert new task into the database.
            match insert_task(&conn, &task) {
                Ok(_) => {
                        println!("Task data inserted successfully!");
                }                            
                Err(e) => {
                    println!("Failed to insert task: {:?}", e);                    
                    return Err(e.into());
                }
            }
            // Initializing employee data directly            
            let employee = Employee {
                id: 2,
                name: "John samuel".to_string(),
                position: Some("Developer".to_string()),
            };
            // Insert new employee into the database.
            match insert_employee(&conn, &employee) {
                Ok(_) => {
                        println!("Employee data inserted successfully!");
                }                            
                Err(e) => {
                    println!("Failed to insert employee: {:?}", e);                    
                    return Err(e.into());
                }
            }            
        }        
        Err(err) => {
            eprintln!("Error creating tables: {}", err);              
            return Err(err);
        }
    }
    Ok(())
}
