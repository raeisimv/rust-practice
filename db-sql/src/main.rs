use std::io::{stdin, Write};

fn main() {
    wall();
    repl();
}

fn repl() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(line)) => {
                if line.trim() == "exit" {
                    println!("exiting ...");
                    break;
                }
                println!("received: {}", line.trim());
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
