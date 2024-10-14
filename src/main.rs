mod db;
mod sql_engine;

use db::btreemap_database::Database;
use db::data_types::{Column, DataType};
use db::table::{Row, Table};
use serde::{Deserialize, Serialize};
use sql_engine::{process_sql, SqlCommand};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

// fn main() -> io::Result<()> {
//     let args: Vec<String> = std::env::args().collect();
//     if args.len() != 2 {
//         eprintln!("Usage: {} <database_name>", args[0]);
//         std::process::exit(1);
//     }
//
//     let db_name = &args[1];
//     let db_path = Path::new("src").join(format!("{}.db", db_name));
//
//     let mut database = if db_path.exists() {
//         println!("Loading existing database: {}", db_path.display());
//         Database::load_from_file(&db_path)?
//     } else {
//         println!("Creating new database: {}", db_path.display());
//         Database::new()
//     };
//
//     let stdin = io::stdin();
//     let mut stdout = io::stdout();
//
//     loop {
//         print!("SQL> ");
//         stdout.flush()?;
//
//         let mut input: String = String::new();
//         stdin.lock().read_line(&mut input)?;
//         let input: &str = input.trim();
//
//         if input.eq_ignore_ascii_case("exit") {
//             break;
//         }
//
//         let lower_input = input.to_lowercase();
//         match process_sql(input) {
//             Ok(command) => match command {
//                 SqlCommand::CreateTable(table_name, columns) => {
//                     database.create_table(table_name.clone(), columns);
//                     println!("Table '{}' created successfully.", table_name);
//                     database.save_to_file(&db_path)?;
//                 }
//                 SqlCommand::Insert(table_name, values) => {
//                     match database.insert_row(&table_name, values) {
//                         Ok(_) => {
//                             println!("Row inserted successfully into table '{}'.", table_name);
//                             database.save_to_file(&db_path)?;
//                         }
//                         Err(e) => println!("Error inserting row: {}", e),
//                     }
//                 }
//                 SqlCommand::Select {
//                     field1: table_name,
//                     field2: columns,
//                 } => {
//                     match database.select(&table_name, &columns) {
//                         Ok(result) => {
//                             // TODO: remove save to file & modify this arm such to  use
//                             // "database.get_table(&table_name)"
//                             database.save_to_file(&db_path)?;
//                         }
//                         Err(e) => println!("Error executing SELECT: {}", e),
//                     }
//                 }
//             },
//             Err(e) => println!("Error processing SQL: {}", e),
//         }
//         // if lower_input.starts_with("create table") {
//         //     match parse_create_table(input) {
//         //         Some((table_name, columns)) => {
//         //             database.create_table(table_name.clone(), columns);
//         //             println!("Table '{}' created successfully.", table_name);
//         //             database.save_to_file(&db_path)?;
//         //         }
//         //         None => println!("Invalid CREATE TABLE command."),
//         //     }
//         // } else if lower_input.starts_with("insert into") {
//         //     match Repl::parse_insert(input) {
//         //         Some((table_name, values)) => match database.insert_row(&table_name, values) {
//         //             Ok(_) => {
//         //                 println!("Row inserted successfully into table '{}'.", table_name);
//         //                 database.save_to_file(&db_path)?;
//         //             }
//         //             Err(e) => println!("Error inserting row: {}", e),
//         //         },
//         //         None => println!("Invalid INSERT command."),
//         //     }
//         // } else if lower_input.starts_with("select") {
//         //     match Repl::parse_select(input) {
//         //         Some((table_name, columns)) => match database.select(&table_name, &columns) {
//         //             Ok(result) => {
//         //                 result.pretty_print();
//         //                 database.save_to_file(&db_path)?;
//         //             }
//         //             Err(e) => println!("Error executing SELECT: {}", e),
//         //         },
//         //         None => println!("Invalid SELECT command."),
//         //     }
//         // } else {
//         //     println!("Unrecognized command. Supported commands: CREATE TABLE, INSERT INTO, SELECT");
//         // }
//     }
//
//     database.save_to_file(&db_path)?;
//     println!("Database saved. Exiting.");
//     Ok(())
// }

#[allow(unused_variables)]
fn main() -> io::Result<()> {
    // To run application use cargo run -- my_database_name
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprint!("Main: invalid number of arguments.");
        std::process::exit(1);
    }

    let db_name = &args[1];
    let db_path = Path::new("src").join(format!("{}.db", db_name));

    let mut database = if db_path.exists() {
        println!("Main: loading existing database: {}", db_path.display());
        Database::load_from_file(&db_path)?
    } else {
        println!("Main: creating new database: {}", db_path.display());
        Database::new()
    };

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("SQL> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        match process_sql(input) {
            Ok(command) => match execute_command(&mut database, command) {
                Ok(message) => {
                    println!("{}", message);
                    database.save_to_file(&db_path)?;
                }
                Err(e) => println!("Main: error executing command: {}", e),
            },
            Err(e) => println!("Main: error processing SQL: {}", e),
        }
    }

    database.save_to_file(&db_path)?;
    println!("Database saved. Exiting.");
    Ok(())
}

fn execute_command(database: &mut Database, command: SqlCommand) -> Result<String, String> {
    match command {
        SqlCommand::CreateTable { name, columns } => {
            database.create_table(name.clone(), columns);
            Ok(format!("Main: table '{}' created successfully.", name))
        }
        SqlCommand::Insert {
            table,
            columns,
            values,
        } => {
            // TODO: fix method
            // database.insert_row(&table, values)?;
            Ok(format!(
                "Main: row inserted successfully into table '{}'.",
                table
            ))
        }
        SqlCommand::Select {
            table,
            columns,
            where_clause,
            join_clause,
        } => {
            let result = database.select(&table, &columns)?;
            // Assuming you have a function to pretty print the result
            // TODO: either put the logic in here to print the table, leave logic in the table, or
            // return the table
            // print_result(&result, &columns);
            Ok("Main: query executed successfully.".to_string())
        }
    }
}
