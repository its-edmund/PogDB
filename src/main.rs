use chrono;
use std::io::Write;

fn main() {
    println!(
        "{}> PogDB is starting",
        chrono::offset::Local::now()
            .format("%d %b %Y %H:%M:%S")
            .to_string()
    );

    println!("");
    println!("###################################################");
    println!("##### ________              ________________  #####");
    println!("##### ___  __ \\____________ ___  __ \\__  __ ) #####");
    println!("##### __  /_/ /  __ \\_  __ `/_  / / /_  __  | #####");
    println!("##### _  ____// /_/ /  /_/ /_  /_/ /_  /_/ /  #####");
    println!("##### /_/     \\____/_\\__, / /_____/ /_____/   #####");
    println!("#####               /____/                    #####");
    println!("###################################################");
    println!("");

    let mut command = String::new();

    loop {
        print!(
            "{}> ",
            chrono::offset::Local::now()
                .format("%d %b %Y %H:%M:%S")
                .to_string()
        );
        std::io::stdout().flush().ok();
        let _b1 = std::io::stdin()
            .read_line(&mut command)
            .ok()
            .expect("Failed to read line");
        parse_instruction(&command.trim())
    }
}

fn parse_instruction(command: &str) {
    match command {
        "put" => {
            println!("put data");
        }
        "quit" => {
            println!("quit command");
        }
        _ => println!("command not found"),
    }
}
