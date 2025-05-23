mod errors;
mod exec;
mod parser;

use crate::exec::ExecutionContext;
use std::io::{Write, stdin};

fn main() {
    wall();
    repl();
}

//https://cstack.github.io/db_tutorial/parts/part1.html
// https://johns.codes/blog/build-a-db/part01
//https://medium.com/@krizzsrivastava/retr0db-building-a-database-in-rust-b223e2b98cbd

fn repl() {
    let mut ctx = ExecutionContext::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(line)) => {
                if line.trim() == "exit" {
                    println!("exiting ...");
                    break;
                }
                let _ = ctx.run(line.as_str());
            }
            Some(Err(e)) if e.kind() == std::io::ErrorKind::Interrupted => {
                // CTRL + C
                continue;
            }
            Some(Err(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                // CTRL + D
                return;
            }
            Some(Err(e)) => {
                println!("Error: {}", e);
                return;
            }
            _ => {
                println!("received empty line iterator");
                continue;
            }
        }
    }
}

fn wall() {
    println!("Welcome to DB SQL");
    println!(" ** This is a try to create a database for practicing Rust **");
    println!("Version: 0.1.0");
    println!("Help:");
    println!("Just write your SQL Command/Query here");
    println!("-----------------------------------------------------");
}
