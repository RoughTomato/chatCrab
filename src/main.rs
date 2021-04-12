extern crate clap;
use clap::{Arg, App};
use std::io;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

fn handle_server(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    
    while match stream.read(&mut data) {
        Ok(_) => {
            stream.write(b"Ok, thank you for sending data!").unwrap();
            true
        },
        Err(_) => {
            println!("Error: terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn run_server() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_server(stream)
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}

fn client_connect(connection_string: String) {
    match TcpStream::connect(connection_string) {
        Ok(mut stream) => {
            let mut msg = String::new();

            println!("Connected");

            while msg != "q" {
            
                io::stdin()
                .read_line(&mut msg)
                .expect("Failed to read line");
                stream.write(msg.as_bytes()).unwrap();

                let mut data = [0 as u8; 50];
                match stream.read(&mut data) {
                    Ok(_) => {
                        let text = from_utf8(&data).unwrap();
                        println!("{}", text);
                    },
                    Err(e) => {
                        println!("Error! failed to receive data: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Connection terminated");
}

fn main() {
    let matches = App::new("chatCrab")
    .version("v0.0.1")
    .author("Amadeusz Dabkowski <adabkowski93@gmail.com")
    .about("Simple TCP based chat program written in Rust")
    .arg(Arg::with_name("server")
            .short("s")
            .long("server")
            .help("Starts chat in server mode"))
    .arg(Arg::with_name("client")
            .short("c")
            .long("client")
            .takes_value(true)
            .help("Starts chatCrab in client mode, takes server addres
            and port as an argument in xxx.xxx.xxx.xxx:yyyy format"))
    .get_matches();
    //TODO: consider playing with Rusts GUI libs and allow for CLI or GUI selection

    let server = matches.is_present("server");
    let client = matches.is_present("client");

    if server && !client {
        println!("Running chatCrab in server mode");
        run_server();
    } else if client && !server {
        let client_data = matches.value_of("client").unwrap_or("127.0.0.1:3333");
        client_connect(client_data.to_string());
    }

}