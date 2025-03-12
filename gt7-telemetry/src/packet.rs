use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub struct Packet {
    pub car_speed: f32 //in m/s
}

impl Packet {
    pub fn from_bytes(b: &[u8]) -> Result<Self, &'static str> {

        let token = std::str::from_utf8(&b[0..4]).unwrap_or("");
        if token != "0S7G" {
            println!("Got {}", token);
            return Err("Invalid Token!");
        }

        Ok(Packet {
            car_speed: get_float(b, 76)
        })
    }
}

fn get_float(b: &[u8], i: usize) -> f32 {
    LittleEndian::read_f32(&b[i..i + 4])
}
