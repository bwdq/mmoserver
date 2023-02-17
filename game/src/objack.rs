use bytes::Buf;
use crate::message::Message;

pub fn parse_obj_ack_msg(mut msg: Message) -> Vec<objack> {


    let mut acks: Vec<objack> = Vec::new();

    if msg.msg_data.len() > 1000 {
        println!("OBJ_ACK message too long");
        return acks
    }
    while msg.msg_data.len() > 3 {
        let id = msg.msg_data.get_u32();
        let frame = msg.msg_data.get_u32();
        let ack = objack {
            id,
            frame,
        };
        acks.push(ack)
    }
    acks
}


pub struct objack {
    id: u32,
    frame: u32,
}