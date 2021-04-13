extern crate clap;
use clap::{Arg, App};

mod server;
pub use server::crabserver;

mod client;
pub use client::crabclient;

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
        crabserver::run_server("127.0.0.1:3333")
    } else if client && !server {
        let _client_data = matches.value_of("client").unwrap_or("127.0.0.1:3333");

    }

}