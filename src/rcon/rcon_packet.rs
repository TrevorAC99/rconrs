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
    pub fn new(id: i32, cmd: i32, data: &str) -> Self {
        const BASE_PACKET_SIZE: i32 = 8;
        let mut data = Vec::from(data.as_bytes());
        //Add two 0 bytes on the end of the string.
        data.extend_from_slice(&[0u8; 2]);

        let size = BASE_PACKET_SIZE + (data.len() as i32);

        Self {
            size,
            id,
            cmd,
            data,
        }
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
        String::from_utf8_lossy(&self.data)
            .trim()
            .replace("\u{0}", "")
    }
}
