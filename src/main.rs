use structopt::StructOpt;

use rcon::RconClient;
use std::io::{prelude::*, stdin, stdout};

mod rcon;

fn main() {
    let options: Options = Options::from_args();

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
/// executes the ```quit``` command or the ```stop``` command.
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

            if command.eq_ignore_ascii_case("stop") {
                break;
            }

            if !response.is_empty() {
                println!("{}", response);
            }
        }
    }
}

/// The command line options for the program.
#[derive(StructOpt)]
#[structopt(
    author = "Trevor Carlson <trevorac99@gmail.com>",
    about = include_str!("about.txt")
)]
struct Options {
    #[structopt(
        name = "HOST",
        short = "H",
        long = "host",
        default_value = "localhost",
        help = "The host to connect to."
    )]
    host: String,
    #[structopt(
        name = "PORT",
        short = "p",
        long = "port",
        default_value = "25575",
        help = "The port of the RCON server."
    )]
    port: u16,
    #[structopt(
        name = "PASSWORD",
        short = "P",
        long = "password",
        help = "The password of the RCON server."
    )]
    password: String,
}
