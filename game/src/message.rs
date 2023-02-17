use std::net::SocketAddr;
use std::ops::IndexMut;
use bytes::{Buf, BufMut, BytesMut};
use nom::AsBytes;
use tokio::io;
use tokio::net::UdpSocket;

const MSG_SESS: u8 = 0;
const MSG_REL: u8 = 1;
const MSG_ACK: u8 = 2;
const MSG_BEAT: u8 = 3;
const MSG_MAPREQ: u8 = 4;
const MSG_MAPDATA: u8 = 5;
const MSG_OBJDATA: u8 = 6;
const MSG_OBJACK: u8 = 7;
const MSG_CLOSE: u8 = 8;

#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum MessageType {
    Session,
    Reliable,
    Ack,
    Beat,
    MapRequest,
    MapData,
    ObjectData,
    ObjectAck,
    Close,
    Error,
}

impl MessageType {
    pub fn from_u8(val: u8) -> Self {
        match val {
            MSG_SESS => MessageType::Session,
            MSG_REL => MessageType::Reliable,
            MSG_ACK => MessageType::Ack,
            MSG_BEAT => MessageType::Beat,
            MSG_MAPREQ => MessageType::MapRequest,
            MSG_MAPDATA => MessageType::MapData,
            MSG_OBJDATA => MessageType::ObjectData,
            MSG_OBJACK => MessageType::ObjectAck,
            MSG_CLOSE => MessageType::Close,
            _ => MessageType::Error,
        }
    }
}

#[derive(Debug)]
pub struct MessageAddr {
    pub socket_addr: SocketAddr,
    pub msg: Message,
}

impl MessageAddr {
    pub fn new(socket_addr: SocketAddr, msg: Message) -> Self {
        MessageAddr {
            socket_addr,
            msg,
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub msg_type: MessageType,
    pub msg_data: BytesMut,
}

impl Message {
    pub fn new(msg_type: MessageType) -> Self {
        Message {
            msg_type,
            msg_data: BytesMut::new(),
        }
    }

    /// Serialize the message into a byte vector
    pub fn ser(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        match self.msg_type {
            MessageType::Session => {
                buf.push(MSG_SESS);
                buf.extend_from_slice(&self.msg_data);
            }
            MessageType::Reliable => {
                buf.push(MSG_REL);
                buf.extend_from_slice(&self.msg_data);
            }
            MessageType::Ack => {
                buf.push(MSG_ACK);
                buf.extend_from_slice(&self.msg_data);
            }
            MessageType::Beat => {
                dbg!("MSG_BEAT is receive only");
            }
            MessageType::MapRequest => {
                dbg!("MSG_MAPREQ is receive only");
            }
            MessageType::MapData => {
                buf.push(MSG_MAPDATA);
                buf.extend_from_slice(&self.msg_data);
            }
            MessageType::ObjectData => {
                buf.push(MSG_OBJDATA);
                buf.extend_from_slice(&self.msg_data);
            }
            MessageType::ObjectAck => {
                dbg!("MSG_OBJACK is receive only");
            }
            MessageType::Close => {
                buf.push(MSG_CLOSE);
            }
            MessageType::Error => {
                panic!("Error message type cannot be sent");
            }
        }
        buf
    }
}



pub struct PMessage {
    p_type: u8,
    pub(crate) msg: BytesMut,
}

impl PMessage {
    pub fn new(p_type: u8) -> Self {
        PMessage {
            p_type,
            msg: BytesMut::new(),
        }
    }
}


pub struct RMessage {//reliable? remote ui?
    p_msg: PMessage,
    seq: u16,
    last: u64,
    re_tx: u8,
}