use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::net::{TcpStream};

use rcon_packet::RconPacket;
use super::rcon_packet;

const RCON_EXEC_COMMAND: i32 = 2;
const RCON_AUTHENTICATE: i32 = 3;
#[allow(dead_code)]
const RCON_RESPONSEVALUE: i32 = 0;
#[allow(dead_code)]
const RCON_AUTH_RESPONSE: i32 = 2;
const RCON_PID: i32 = 0xDEC0DED;
const RCON_FOLLOW_PID: i32 = 0xB1ADED;

/// An RconClient that is connected and able to execute commands.
pub struct RconClient {
    /// A handle to the TcpStream wrapped in a BufReader.
    reader: BufReader<TcpStream>,
    /// A handle to the TcpStream wrapped in a BufWriter.
    writer: BufWriter<TcpStream>,
}

impl RconClient {
    /// Connects to a host and attempts to authenticate. If the authentication
    /// is successful, it returns an Ok containing an RconClient, otherwise
    /// returning an Err.
    pub fn connect(host: &str, port: u16, password: &str) -> std::io::Result<RconClient> {

        let stream = TcpStream::connect((host, port))?;
    
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream);

        let mut client = RconClient {reader, writer};

        client.auth(password)?;

        Ok(client)
    }

    /// Sends an Authenticate packet with the password. If the response
    /// indicates success, this returns Ok and otherwise returns an
    /// Err.
    fn auth(&mut self, password: &str) -> std::io::Result<()> {
        let auth_packet = RconPacket::new(RCON_PID, RCON_AUTHENTICATE, password);

        self.send_packet(auth_packet)?;

        let response_packet = self.receive_packet()?;

        if response_packet.get_id() == -1 {
            new_io_err("Rcon Authentication Failure")
        } else {
            Ok(())
        }
    }

    /// Executes an arbitrary command. If there are no problems during
    /// execution, it yields an Ok containing the server's response as a
    /// String. If it fails, it yields an Err
    pub fn exec_command(&mut self, command: &str) -> std::io::Result<String> {
        let command_packet = RconPacket::new(RCON_PID, RCON_EXEC_COMMAND, command);
        let follow_up_packet = RconPacket::new(RCON_FOLLOW_PID, RCON_EXEC_COMMAND, "");

        self.send_packet(command_packet)?;
        self.send_packet(follow_up_packet)?;

        let mut response = String::new();

        loop {
            let receive_packet = self.receive_packet()?;

            if receive_packet.get_id() == RCON_PID {
                response.push_str(&receive_packet.get_data_string().trim());
                continue;
            } else if receive_packet.get_id() == RCON_FOLLOW_PID {
                response = String::from(response);
                break Ok(response)
            } else {
                break new_io_err("Rcon Exec Response Id Invalid")
            }
        }
    }

    /// Sends a single packet over the internal TcpStream.
    fn send_packet(&mut self, packet: RconPacket) -> std::io::Result<()> {
        let bytes = packet.into_bytes();

        // Loop until the write is successful or a fatal error is hit.
        loop {
            if let Err(e) = self.writer.write(&bytes) {
                // Interrupted is non-fatal and can safely be retried.
                if let ErrorKind::Interrupted = e.kind() {
                    continue;
                } else {
                    return Err(e);
                }
            } else {
                break
            }
        }

        self.writer.flush()?;

        Ok(())
    }

    /// Receives a single packet from the internal TcpStream
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

trait ReadI32FromLeBytes {
    /// Read an i32 from a byte stream in little endian.
    fn read_i32_from_le_bytes(&mut self) -> std::io::Result<i32>;
}

impl<R: Read> ReadI32FromLeBytes for BufReader<R> {
    fn read_i32_from_le_bytes(&mut self) -> std::io::Result<i32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }
}

/// Simple way to create a Err containing a std::io::Error of kind Other that
/// contains a given message.
fn new_io_err<T>(message: &'static str) -> std::io::Result<T> {
    Err(
        Error::new(
            ErrorKind::Other,
            message
        )
    )
}
