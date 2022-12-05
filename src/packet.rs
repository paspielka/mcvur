use std::net::*;

struct Packet {
    stream: TcpStream,
}

impl Packet {
    pub fn new(server_address: String, port: u16) -> Self {
        let fixed_ip = format!("{}:{}", server_address, port);

        Self {
            stream: match TcpStream::connect(fixed_ip) {
                Ok(s) => s,
                Err(error) => panic!("Failed to connect stream: {:?}", error)
            }
        }
    }

    pub fn status_packet(&self) {
        
    }
}