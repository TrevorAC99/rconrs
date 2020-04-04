use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpStream};

const RCON_EXEC_COMMAND: i32 = 2;
const RCON_AUTHENTICATE: i32 = 3;
#[allow(dead_code)]
const RCON_RESPONSEVALUE: i32 = 0;
#[allow(dead_code)]
const RCON_AUTH_RESPONSE: i32 = 2;
const RCON_PID: i32 = 0xDEC0DED;
const RCON_FOLLOW_PID: i32 = 0xB1ADED;

/// Represents an RCON packet which can be serialized into raw bytes to be
/// sent over the network.
pub struct RconPacket {
    /// The length in bytes of the rest of the packet.
    size: i32,
    /// The id of the packet. The id of a response packet while be the same as
    /// the id of the packet it is responding to.
    id: i32,
    /// The type of command represented by this packet. For outgoing packets,
    /// this will either be 2 (for a command execution) or 3 (for an
    /// authentication) attempt.
    cmd: i32,
    /// The data being sent as a sequence of bytes. This should be a null-
    /// terminated ASCII string followed by an empty null-terminated ASCII
    /// string aka another 0 byte. This is basically a sequence of ASCII codes
    /// followed by two 0 (null) bytes.
    data: Vec<u8>,
}

impl RconPacket {
    /// Creates a new RconPacket based on the given data.
    pub fn new(id: i32, cmd: i32, data: &str) -> RconPacket {
        const BASE_PACKET_SIZE: i32 = 8;
        let mut data = Vec::from(data.as_bytes());
        //Add two 0 bytes on the end of the string.
        data.extend_from_slice(&[0u8;2]);
        
        let size = BASE_PACKET_SIZE + (data.len() as i32);

        RconPacket {size, id, cmd, data}
    }

    /// Turns the packet into raw bytes to be sent over the network.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.cmd.to_le_bytes());
        bytes.extend_from_slice(&self.data);

        bytes
    }

    /// Gets the size value of the packet.
    #[allow(dead_code)]
    pub fn get_size(&self) -> i32 {
        self.size
    }

    /// Gets the id value of the packet.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Gets the command code of the packet.
    #[allow(dead_code)]
    pub fn get_cmd(&self) -> i32 {
        self.cmd
    }

    /// Gets the data of the packet as a slice of bytes.
    #[allow(dead_code)]
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Gets the data of the packet and creates a String from it.
    pub fn get_data_string(&self) -> String {
        String::from(String::from_utf8_lossy(&self.data).trim())
    }
}

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

        let stream = TcpStream::connect((host, port)).unwrap();
    
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);

        let mut client = RconClient {reader, writer};

        client.auth(password)?;

        Ok(client)
    }

    /// Sends an Authenticate packet with the password. If the response
    /// indicates success, this returns Ok and otherwise returns an
    /// Err.
    pub fn auth(&mut self, password: &str) -> std::io::Result<()> {
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
                response.push_str(&receive_packet.get_data_string());
                continue;
            } else if receive_packet.get_id() == RCON_FOLLOW_PID {
                break Ok(response)
            } else {
                break new_io_err("Rcon Exec Response Id Invalid")
            }
        }
    }

    /// Sends a single packet over the internal TcpStream
    fn send_packet(&mut self, packet: RconPacket) -> std::io::Result<()> {
        let bytes = packet.into_bytes();

        self.writer.write(&bytes)?;
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
        std::io::Error::new(
            std::io::ErrorKind::Other,
            message
        )
    )
}
