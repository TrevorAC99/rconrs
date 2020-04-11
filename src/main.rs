#[macro_use]
extern crate clap;

use rcon::RconClient;
use std::io::{prelude::*, stdin, stdout};

mod rcon;

const DEFAULT_HOST: &'static str = "localhost";
const DEFAULT_PORT: u16 = 25575;

fn main() {
    let options = get_options();

    print!("Connecting... ");

    let client = match RconClient::connect(&options.host, options.port, &options.password) {
        Err(_) => {
            stdout().flush().unwrap();
            eprintln!("Unable to connect. Did you enter your password correctly? Shutting down.");
            return;
        }
        Ok(client) => client,
    };

    println!("Connected");

    command_loop(client);
}

/// Takes input from the user and executes it as commands until the user
/// executes the ```quit``` command.
fn command_loop(mut client: RconClient) {
    let input = stdin();
    loop {
        print!("Enter a command: ");
        stdout().flush().unwrap();

        let mut command_buffer = String::new();
        input.read_line(&mut command_buffer).unwrap();
        let command = command_buffer.trim();

        if command.eq_ignore_ascii_case("quit") {
            println!("Exiting");
            break;
        } else {
            let response = match client.exec_command(command) {
                Err(_) => {
                    println!("Error: unable to execute the command.");
                    continue;
                }
                Ok(response) => response,
            };

            if !response.is_empty() {
                println!("{}", response);
            }
        }
    }
}

/// The command lines options for the program.
struct Options {
    host: String,
    port: u16,
    password: String,
}

/// Collects the arguments from the command line into the ```Options```
/// struct.
fn get_options() -> Options {
    let matches = clap_app!(rconrs =>
        (version: "0.1.0")
        (author: "Trevor Carlson <trevorac99@gmail.com>")
        (about: include_str!("about.txt"))
        (@arg HOST: -H --host +takes_value "Sets the host to connect to. Leave blank for localhost.")
        (@arg PORT: -p --port +takes_value "Sets the port to connect to. Leave blank for 25575.")
        (@arg PASSWORD: -P --password +takes_value +required "The password of the RCON server")
    ).get_matches();

    let host = matches.value_of("HOST").unwrap_or(DEFAULT_HOST);
    let host = String::from(host);

    let port = match matches.value_of("PORT") {
        Some(port) => match port.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                print!("Invalid value for port. Using {} instead.", DEFAULT_PORT);
                DEFAULT_PORT
            }
        },
        None => DEFAULT_PORT,
    };

    let password = matches.value_of("PASSWORD").unwrap();
    let password = String::from(password);

    Options {
        host,
        port,
        password,
    }
}
