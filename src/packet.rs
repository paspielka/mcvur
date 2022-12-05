use std::net::*;

use crate::pack::Pack;

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
        let packet = Pack::new(0)
            .pack_int(760)
            .pack_string("127.0.0.1")
            .pack_short(25565)
            .pack_int(1)
            .build();

        
    }
}