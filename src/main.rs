use std::io::prelude::*;

mod rcon;

fn main() {
    let input = std::io::stdin();
    let mut input = input.lock();

    println!("{}", include_str!("greeting.txt"));

    print!("Enter the host you wish to connect to or leave blank for localhost: ");
    std::io::stdout().flush().unwrap();

    let mut host_buffer = String::new();
    input.read_line(&mut host_buffer).unwrap();

    let host = if host_buffer.trim().eq("") {
        "localhost"
    } else {
        host_buffer.trim()
    };

    let mut port_buffer = String::new();

    print!("Enter the port you wish to use or leave blank for 25575: ");
    std::io::stdout().flush().unwrap();
    let port = loop {
        input.read_line(&mut port_buffer).unwrap();
        let port_trimmed = port_buffer.trim();

        if port_trimmed.is_empty() { break 25575 }

        match port_trimmed.parse::<u16>() {
            Ok(port) => { break port }
            Err(_) => {
                print!("Unable to parse input. Enter the port you wish to use or leave blank for 25575: ");
                std::io::stdout().flush().unwrap();
                continue
            }
        }
    };

    print!("Enter the password: ");
    std::io::stdout().flush().unwrap();

    let mut password_buffer = String::new();
    input.read_line(&mut password_buffer).unwrap();

    let password = password_buffer.trim();

    print!("Attempting to connect... ");

    let mut client = match rcon::RconClient::connect(host, port, password) {
        Err(_) => {
            println!("Unable to authenticate. Did you enter your password correctly? Shutting down.");
            return;
        },
        Ok(client) => client
    };

    println!("Authentication successful");

    loop {
        print!("Enter a command: ");
        std::io::stdout().flush().unwrap();
        
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
