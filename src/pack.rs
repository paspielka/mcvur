const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub struct Pack {
    data_buf: Vec<u8>,
}

impl Pack {
    pub fn new(packet_id: i32) -> Self {
        let mut build = Self {
            data_buf: Vec::new(),
        };

        build.pack_int(packet_id);
        build
    }
    // compress variables
    pub fn pack_int_impl(buf: &mut Vec<u8>, mut value: i32) {
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                buf.push(value as u8);
                return;
            }

            buf.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);
            value >>= 7;
        }
    }

    pub fn pack_int(&mut self, value: i32) -> &mut Self {
        Self::pack_int_impl(&mut self.data_buf, value);

        self
    }

    pub fn pack_string(&mut self, value: &str) -> &mut Self {
        self.pack_int(value.len() as i32);
        self.data_buf.extend_from_slice(value.as_bytes());

        self
    }

    pub fn pack_short(&mut self, value: u16) -> &mut Self {
        self.data_buf.extend_from_slice(&value.to_be_bytes());

        self
    }

    // uncompress variables
    pub fn unpack_int(&mut self, value: i32) {

    }

    // return builder
    pub fn build(&mut self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];

        Self::pack_int_impl(&mut data, self.data_buf.len() as i32);
        data.append(&mut self.data_buf);

        data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_pack_int() {
        let mut pack = Pack::new(0);
        let expected:Vec<u8> = vec![221, 199, 1];

        pack.pack_int(25565);
        assert_eq!(pack.data_buf[1..4], expected)
    }

    #[test]
    pub fn test_pack_string() {
        let mut pack = Pack::new(0);
        let expected: Vec<u8> = vec![116, 101, 115, 116];

        pack.pack_string("test");
        assert_eq!(pack.data_buf[2..6], expected);
    }
}