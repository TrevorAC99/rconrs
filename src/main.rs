use std::io::{stdin, stdout, prelude::*};
use rcon::RconClient;

mod rcon;

fn main() {
    println!("{}", include_str!("greeting.txt"));

    let host = read_host();

    let port = read_port();

    let password = read_password();

    print!("Attempting to connect... ");

    let client = match rcon::RconClient::connect(&host, port, &password) {
        Err(_) => {
            println!("Unable to authenticate. Did you enter your password correctly? Shutting down.");
            return;
        },
        Ok(client) => client
    };

    println!("Authentication successful");

    command_loop(client);
}

/// Asks the user for a host name and reads it into a string.
fn read_host() -> String {
    print!("Enter the host you wish to connect to or leave blank for localhost: ");
    stdout().flush().unwrap();

    let mut host_buffer = String::new();
    stdin().read_line(&mut host_buffer).unwrap();

    let host = if host_buffer.trim().eq("") {
        "localhost"
    } else {
        host_buffer.trim()
    };

    String::from(host)
}

/// Asks the user for a port number and reads it into a u16.
fn read_port() -> u16 {
    let mut port_buffer = String::new();

    print!("Enter the port you wish to use or leave blank for 25575: ");
    stdout().flush().unwrap();

    loop {
        stdin().read_line(&mut port_buffer).unwrap();
        let port_trimmed = port_buffer.trim();

        if port_trimmed.is_empty() { break 25575 }

        match port_trimmed.parse::<u16>() {
            Ok(port) => { break port }
            Err(_) => {
                print!("Unable to parse input. Enter the port you wish to use or leave blank for 25575: ");
                stdout().flush().unwrap();
                continue
            }
        }
    }
}

/// Asks the user for a password and reads it into a String.
fn read_password() -> String {
    print!("Enter the password: ");
    stdout().flush().unwrap();

    let mut password_buffer = String::new();
    stdin().read_line(&mut password_buffer).unwrap();

    String::from(password_buffer.trim())
}

/// Takes commands from the user and executes them as commands until the user
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
                },
                Ok(response) => response
            };

            if !response.is_empty() {
                println!("{}", response);
            }
        }
    }
}
