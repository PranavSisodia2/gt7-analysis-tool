mod packet;
mod salsa_decrypt;
mod socket_connection;

use serde::{Serialize, Deserialize};
use zmq;
use byteorder::{ByteOrder};
use dotenv::dotenv;

fn main() {

    dotenv().ok();
    sodiumoxide::init().unwrap();

    let socket = socket_connection::init_socket_connection();

    loop {
        let packet = socket_connection::recv_data(&socket);
        println!("Speed: {} km/h", packet.car_speed * 3.6);
    }

}


