use crate::salsa_decrypt;
use std::net::UdpSocket;
use zmq;
use std::env;
use crate::packet::Packet;
use std::thread;
use std::time::Duration;

const HEARTBEAT_PORT: u32 = 33739;
const BIND_PORT: u16 = 33740;
const BUFFER_LEN: usize = 0x128;
const HEARTBEAT_DELAY: u64 = 10; // secs
const HEARTBEAT_MESSAGE: &[u8] = b"A";


pub fn init_socket_connection() -> UdpSocket {

    let bind_address = format!("0.0.0.0:{}", BIND_PORT);
    let socket = UdpSocket::bind(&bind_address).unwrap();

    println!("Listening on {}...", BIND_PORT);

    let socket_clone = socket.try_clone().unwrap();
    let _ = thread::spawn(move || heartbeat(&socket_clone));

    socket
}

pub fn recv_data(socket: &UdpSocket) -> Packet {

    let mut buffer = [0; BUFFER_LEN];
    let _ = socket.recv_from(&mut buffer);
    salsa_decrypt::decrypt(&mut buffer);

    Packet::from_bytes(&buffer).unwrap()
}

fn heartbeat(socket: &UdpSocket) {
    let ps5_ip_address = env::var("PS5_IP_ADDRESS").expect("Set PS5 IP in .env file.");
    let heartbeat_address = format!("{}:{}", ps5_ip_address, HEARTBEAT_PORT);

    loop {
        socket.send_to(HEARTBEAT_MESSAGE, &heartbeat_address).unwrap();
        thread::sleep(Duration::from_secs(HEARTBEAT_DELAY));
    }
}



