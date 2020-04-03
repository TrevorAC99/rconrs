use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpStream};

const RCON_EXEC_COMMAND: i32 = 2;
const RCON_AUTHENTICATE: i32 = 3;
//const RCON_RESPONSEVALUE: i32 = 0;
//const RCON_AUTH_RESPONSE: i32 = 2;
const RCON_PID: i32 = 0xDEC0DED;

fn main() {
    let input = std::io::stdin();
    let mut input = input.lock();

    print!("Enter the host you wish to connect to or leave blank for localhost: ");
    std::io::stdout().flush().unwrap();

    let mut host_buffer = String::new();
    input.read_line(&mut host_buffer).unwrap();

    let host = if host_buffer.trim().eq("") { "localhost" } else { host_buffer.trim() };

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

    print!("Attempting to connect...");

    let mut client = RconClient::connect(host, port, password).unwrap();

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
            let response = client.exec_command(command).unwrap();
            println!("{}", response);
        }
    }
}

struct RconPacket {
    size: i32,
    id: i32,
    cmd: i32,
    data: Vec<u8>,
}

impl RconPacket {
    fn new(id: i32, cmd: i32, data: &str) -> RconPacket {
        const BASE_PACKET_SIZE: i32 = 8;
        let mut data = Vec::from(data.as_bytes());
        //Add two 0 bytes on the end of the string.
        data.extend_from_slice(&[0u8;2]);
        
        let size = BASE_PACKET_SIZE + (data.len() as i32);

        RconPacket {size, id, cmd, data}
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.cmd.to_le_bytes());
        bytes.extend_from_slice(&self.data);

        bytes
    }

    // fn get_size(&self) -> i32 {
    //     self.size
    // }

    fn get_id(&self) -> i32 {
        self.id
    }

    // fn get_cmd(&self) -> i32 {
    //     self.cmd
    // }

    // fn get_data(&self) -> &[u8] {
    //     &self.data
    // }

    fn get_data_string(&self) -> String {
        String::from(String::from_utf8_lossy(&self.data))
    }
}

struct RconClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    packet_id: i32,
}

impl RconClient {
    fn connect(host: &str, port: u16, password: &str) -> std::io::Result<RconClient> {

        let stream = TcpStream::connect((host, port)).unwrap();
    
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);

        let mut client = RconClient {reader, writer, packet_id: RCON_PID};

        client.auth(password)?;

        Ok(client)
    }

    fn auth(&mut self, password: &str) -> std::io::Result<()> {
        let auth_packet = RconPacket::new(self.packet_id, RCON_AUTHENTICATE, password);

        self.send_packet(auth_packet)?;

        let response_packet = self.receive_packet()?;

        if response_packet.get_id() == -1 {
            new_io_err("Rcon Authentication Failure")
        } else {
            Ok(())
        }
    }

    fn exec_command(&mut self, command: &str) -> std::io::Result<String> {
        let command_packet = RconPacket::new(RCON_PID, RCON_EXEC_COMMAND, command);

        self.send_packet(command_packet)?;

        let receive_packet = self.receive_packet()?;

        if receive_packet.get_id() != self.packet_id {
            new_io_err("Rcon Exec Response Id Invalid")
        } else {
            Ok(receive_packet.get_data_string())
        }
    }

    fn send_packet(&mut self, packet: RconPacket) -> std::io::Result<()> {
        let bytes = packet.into_bytes();

        self.writer.write(&bytes)?;
        self.writer.flush()?;

        Ok(())
    }

    fn receive_packet(&mut self) -> std::io::Result<RconPacket> {
        let packet_size = self.reader.read_i32_from_le_bytes()?;
        let packet_id = self.reader.read_i32_from_le_bytes()?;
        let packet_type = self.reader.read_i32_from_le_bytes()?;

        let mut buf = Vec::new();
        buf.resize(packet_size as usize - 8, 0u8);
        self.reader.read_exact(&mut buf)?;

        let data = String::from_utf8_lossy(&buf);
        let data = data.trim();

        let packet = RconPacket::new(packet_id, packet_type, data);
        Ok(packet)
    }
}

impl<R: Read> ReadI32FromLeBytes for BufReader<R> {
    fn read_i32_from_le_bytes(&mut self) -> std::io::Result<i32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }
}

trait ReadI32FromLeBytes {
    fn read_i32_from_le_bytes(&mut self) -> std::io::Result<i32>;
}

fn new_io_err<T>(message: &'static str) -> std::io::Result<T> {
    Err(
        std::io::Error::new(
            std::io::ErrorKind::Other,
            message
        )
    )
}
