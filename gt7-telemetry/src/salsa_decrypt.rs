use byteorder::{ByteOrder, LittleEndian};
use sodiumoxide::crypto::stream::salsa20;

//Referenced: https://github.com/lucaspettit/telempy/blob/main/src/granturismo/security/decrypter.py

const KEY: &[u8] = b"Simulator Interface Packet GT7 v";
const IV_MASK: u32 = 0xDEADBEAF;

pub fn decrypt(data: &mut [u8]) {

    let iv1 = LittleEndian::read_u32(&data[0x40..0x44]);
    let iv2 = iv1 ^ IV_MASK;

    let mut iv = [0u8; 8];
    LittleEndian::write_u32(&mut iv[0..4], iv2);
    LittleEndian::write_u32(&mut iv[4..8], iv1);

    let nonce = salsa20::Nonce::from_slice(&iv).unwrap();

    let key = salsa20::Key::from_slice(KEY).unwrap();

    salsa20::stream_xor_inplace(data, &nonce, &key);
}