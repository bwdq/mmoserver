use std::net::SocketAddr;
use crate::global_state::global;
use crate::message::{MessageType, Message, MessageAddr};

pub fn send_char_create(global: &global, socket_addr: SocketAddr) {

    let msg = Message::new(MessageType::ObjectData);
    let send = MessageAddr::new(socket_addr, msg);


}

enum ObjectType {

}

struct Object {

}

struct ObjectMsg {

    fl: u8,
    id: u32,
    frame: i32,
}

