const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub struct Pack {
    data_buf: Vec<u8>
}

impl Pack {
    pub fn new() -> Self {
        Self {
            data_buf: Vec::new()
        }
    }
    // compress variables
    pub fn pack_int(&mut self, mut value: i32) -> &mut Self{
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                self.data_buf.push(value as u8);
                return self;
            }

            self.data_buf.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);
            value >>= 7;
        }
    }

    pub fn pack_string(&mut self, value: &str) {
        self.pack_int(value.len() as i32);
        self.data_buf.extend_from_slice(value.as_bytes());
    }

    pub fn pack_short(&mut self, value: u16) {
        self.data_buf.extend_from_slice(&value.to_be_bytes())
    }

    // uncompress variables
    pub fn unpack_int(&mut self, value: i32) {

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_pack_int() {
        let mut pack = Pack::new();
        let expected:Vec<u8> = vec![221, 199, 1];

        pack.pack_int(25565);
        assert_eq!(pack.data_buf, expected)
    }

    #[test]
    pub fn test_pack_string() {
        let mut pack = Pack::new();
        let expected: Vec<u8> = vec![116, 101, 115, 116];

        pack.pack_string("test");
        assert_eq!(pack.data_buf[1..5], expected);
    }
}