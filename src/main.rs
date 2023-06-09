mod http_server;

use chrono;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    process,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    log("PogDB is starting");

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

    let command = &mut String::new();
    let stdin = std::io::stdin();

    init();

    let data_store = OpenOptions::new()
        .append(true)
        .open("datastore.csv")
        .ok()
        .unwrap();

    let hash_table = HashMap::new();

    /* for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    } */

    loop {
        print!(
            "{}> ",
            chrono::offset::Local::now()
                .format("%d %b %Y %H:%M:%S")
                .to_string()
        );
        command.clear();
        std::io::stdout().flush().ok();
        let _b1 = stdin.read_line(command);
        parse_instruction(&command.trim(), &data_store);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = "<h1>Welcome to PogDB</h1>";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn put(key: &str, value: &str, mut data_store: &File) {
    writeln!(data_store, "{},{}", key, value);
}

fn print_data_store(data_store: &File) -> Result<(), Box<dyn Error>> {
    let path = Path::new("datastore.csv");
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let tokens = line?;
        let split_tokens = tokens.split(",");
        for token in split_tokens {
            print!("|");
            print!(" {} ", token);
        }
        print!("|\n");
    }
    Ok(())
}

fn parse_instruction(command: &str, data_store: &File) {
    let tokens: Vec<&str> = command.split(" ").collect();
    match tokens[0] {
        "put" => {
            if tokens.len() != 3 {
                println!("Incorrect usage: put requires 2 arguments");
                return;
            }
            put(tokens[1], tokens[2], &data_store);
        }
        "quit" => {
            exit(0);
        }
        "ls" => {
            if let Err(err) = print_data_store(&data_store) {
                eprintln!("Error: {}", err);
                exit(1);
            }
        }
        "help" => {
            println!("help!!! ahhhh!!! help me please!! please!!");
        }
        _ => println!("command not found"),
    }
}

fn log(message: &str) {
    println!(
        "{}> {}",
        chrono::offset::Local::now()
            .format("%d %b %Y %H:%M:%S")
            .to_string(),
        message
    );
}

fn init() -> () {
    match File::create("datastore.csv") {
        Ok(..) => {
            log("File created successfully");
        }
        Err(error) => {
            eprintln!("Error creating datastore: {}", error);
            exit(1);
        }
    }
}

fn exit(error_code: u8) {
    log(&format!(
        "Exiting application with error code {}. Goodbye!",
        error_code
    )
    .to_string());
    process::exit(error_code.into());
}
