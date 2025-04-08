use std::io::{stdin, Write};

fn main() {
    repl()
}

fn repl() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(line)) => {
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
