use std::env;
use std::io::stdin;
use systemd::journal;

fn main() {
    let args: Vec<String> = env::args().collect();
    let separator = match args.get(1) {
        None => ";",
        Some(inner) => inner,
    };

    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_len) => {
                let split = input.split(&separator);
                let vec: Vec<&str> = split.collect();
                journal::send(&vec);
            } 
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }
}
