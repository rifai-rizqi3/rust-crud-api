use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

// Model: User struck with id, name and email
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// Database connection
const DB_URL: &str = !env("DATABASE_URL");

// constants
const OK_RESPONSE : &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND : &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR : &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

// Main function
fn main() {
    // Set DB
    if let Err(e) = set_database() {
        println!("Error setting up database: {}", e);
        return;
    }

    // Start the server and print message
    let listener = TcpListener::bind(format!(0.0.0.0:8090)).unwrap();
    println!("Server started at port 8090");

    // handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(&mut stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

// set_database function
fn set_database() -> Resul<(), PostgresError> {
    // Connect to the database
    let mut client = Client::connect(DB_URL, NoTls)?;
    //Create Table
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        &[]
    )?;

    // Start the server
    start_server(client)
}

// get_id function
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwarp_or_default().split_whitespace().next().unwarp_or_default()
}

// deserialize user from request body with the id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last(1).unwarp_or_default())
}