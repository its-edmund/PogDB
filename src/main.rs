mod http_server;

use chrono;
use clap::Parser;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    process,
};

// ** Command arguments **

#[derive(Parser, Debug)]
#[command(name = "rustdb")]
#[command(author = "Edmund Xin <edmund@gatech.edu>")]
#[command(version = "0.0.1")]
#[command(about = "Fast key-value store built with Rust", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 7878)]
    port: u16,
}

// ** ENTRY POINT **

fn main() {
    let args = Args::parse();

    const HOST: &str = "127.0.0.1";
    let port: u16 = args.port;
    let listener = TcpListener::bind(format!("{}:{}", HOST, port)).unwrap();
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

    log(format!("Server running host {} on port {}", HOST, port).as_str());

    let command = &mut String::new();
    let stdin = std::io::stdin();

    init();

    let data_store = OpenOptions::new()
        .append(true)
        .open("datastore.csv")
        .ok()
        .unwrap();

    let mut hash_table: HashMap<String, String> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Err(e) => print!("ERROR: {}", e),
            Ok(stream) => {
                log("Request received");
                handle_connection(stream, &mut hash_table);
            }
        }
    }

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

fn handle_connection(mut stream: TcpStream, hash_table: &mut HashMap<String, String>) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request: Vec<&str> = http_request[0].split(" ").collect();

    let mut contents = String::from("");
    let mut status_line = "HTTP/1.1 200 OK";

    match request[0] {
        "GET" => {
            log(format!("GET request with payload {}", request[1]).as_str());
            let key = &request[1][1..];
            if key.is_empty() {
                for (i, (key, value)) in hash_table.iter().enumerate() {
                    if i == 0 {
                        contents = format!("{}: {}", key.as_str(), value.as_str());
                    } else {
                        contents = format!("{}\n{}: {}", contents, key.as_str(), value.as_str());
                    }
                }
            } else if !hash_table.contains_key(key) {
                log(format!("Key {} not found in database", key).as_str());
                status_line = "HTTP/1.1 400 BAD REQUEST";
                contents = String::from("Key not found");
            } else {
                contents = hash_table.get(key).unwrap().to_owned();
            }
        }
        "POST" => {
            log(format!("POST request with payload {}", request[1]).as_str());
            let new_entry: Vec<&str> = request[1].split("/").collect();
            if new_entry.len() != 3 {
                log("Two params required for POST request");
                status_line = "HTTP/1.1 400 BAD REQUEST";
                contents = String::from("Two params required for POST request");
            } else {
                hash_table.insert(new_entry[1].to_string(), new_entry[2].to_string());
            }
        }
        _ => {
            status_line = "HTTP/1.1 400 BAD REQUEST";
            contents = String::from("Method not allowed");
        }
    }

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
        "{} {}",
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
