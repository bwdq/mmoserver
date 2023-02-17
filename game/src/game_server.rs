
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use dashmap::DashMap;
use sqlx::SqlitePool;
use tokio::io;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;




use crate::msg_util::{get_next_32_bytes, to_string_ascii, to_string_ascii_2, get_next_string_utf8, parse_nul_terminated_ascii};
use crate::character::Character;
use crate::coord::Coord;
use crate::global_state::global;
use crate::session::{Session, SessionError};
use crate::message::{MessageType, Message, MessageAddr};
//max 16 bit = 65536

const MTU_SIZE: usize = 1500;

const PVER: u8 = 26;

pub async fn run_ipv4(pool: &SqlitePool, global: Arc<global>) -> io::Result<()> {

    //let socket = UdpSocket::bind("127.0.0.1:1870").await?;
    //let socket = Arc::new(UdpSocket::bind("127.0.0.1:1870").await?);
    loop {
        let mut buf = [0u8; MTU_SIZE];
        //let mut buf = BytesMut::with_capacity(MTU_SIZE);//panics
        //let mut buf = vec![0; 1024];
        //let (size, src) = socket.recv_from(&mut buf).await?;
        //let (size, src) = socket.recv_from(&mut buf).await?;
        let (size, src) = global.sock_v4.recv_from(&mut buf).await?;
        println!("Bytes len: {:?} from {:?}", size, src);
        println!("received: {:?}", &buf[0..size]);
        //MSG_? Haven 26(version) username 32 bytes cookie
        println!("as string: {}", to_string_ascii_2(&buf[0..size]));

        //let mut buf_mut = Bytes::from(buf[0..size].to_vec());
        //let socket = socket.clone();
        let global = global.clone();
        //println!("glob2: {:?}", glob2);
        tokio::spawn(async move {
            //process(&buf[0..size], src, &socket, &global).await;
            process(&buf[0..size], src, &global.sock_v4, &global).await;
        });
    }
    Ok(())
}



// pub fn process(mut buf: &[u8], src: SocketAddr, socket: &UdpSocket, global: &global) {
pub async fn process(mut buf: &[u8], src: SocketAddr, socket: &UdpSocket, global: &global) {
    if buf.len() < 1 {
        return;
    }
    let msg_type = MessageType::from_u8(buf.get_u8());

    let mut msg = Message::new(msg_type);
    msg.msg_data.put_slice(&buf[..]);
    let mut msg_addr = MessageAddr::new(src, msg);
    match msg_addr.msg.msg_type {
        MessageType::Session => {
            println!("msg sess");
            parse_session(msg_addr, global).await;
        }
        MessageType::Reliable => {
            println!("msg rel");
            parse_reliable(msg_addr);
        }
        MessageType::Ack => {
            println!("msg ack");
            parse_ack(msg_addr);
        }
        MessageType::Beat => {
            println!("msg beat");
            parse_beat(msg_addr, global);
        }
        MessageType::MapRequest => {
            println!("msg mapreq");
            parse_map_req(msg_addr);
        }
        MessageType::ObjectAck => {
            println!("msg objack");
            parse_obj_ack(msg_addr);
        }
        MessageType::Close => {
            println!("msg close");
            close_sess(msg_addr.socket_addr);
        }
        _ => {
            dbg!(" {} message type should not be sent by client", msg_addr.msg.msg_type);
        }
    }
}

pub async fn parse_session (mut msg_addr: MessageAddr, global: &global) {
    if msg_addr.msg.msg_data.len() < 3 {
        return;
    }
    let seq = msg_addr.msg.msg_data.get_u16_le();

    //if return error then return from function
    let protocol = get_next_string_utf8(&mut msg_addr.msg.msg_data);
    if protocol.is_err() {
        return;
    }
    let protocol = protocol.unwrap();

    //let protocol = get_next_string_utf8(&mut msg.msg_data).unwrap();
    let pver = msg_addr.msg.msg_data.get_u16_le();
    let username = get_next_string_utf8(&mut msg_addr.msg.msg_data);
    if username.is_err() {
        return;
    }
    let username = username.unwrap();
    let cookie_len = msg_addr.msg.msg_data.get_u16_le();
    let cookie = get_next_32_bytes(&mut msg_addr.msg.msg_data).unwrap();
    let session_req = SessionReq {
        seq,
        protocol,
        pver,
        username,
        cookie_len,
        cookie,
    };
    println!("session: {:?}", session_req);
    let session = Session::new(session_req.username.to_string(), msg_addr.socket_addr);
    let res = global.add_session(msg_addr.socket_addr, session).await;
    println!("res: {:?}", res);
    let mut rep = Message::new(MessageType::Session);
    rep.msg_data.put_u8(res.as_u8());
    let rep_addr = MessageAddr::new(msg_addr.socket_addr, rep);
    global.send_msg(rep_addr).await;
}

pub fn parse_reliable (mut msg: MessageAddr) {
    if msg.msg.msg_data.len() < 3 {
        return;
    }
    let seq = msg.msg.msg_data.get_u16();
    if msg.msg.msg_data.len() < 2 {
        return;
    }
    let msg_type = msg.msg.msg_data.get_u8();
    if msg.msg.msg_data.len() < 1 {
        return;
    }
    println!("end rel parse")
}

pub fn parse_ack (mut msg: MessageAddr) {

}

pub fn parse_beat (mut msg: MessageAddr, global: &global) {
    global.receive_msg(msg);
}

pub fn parse_map_req(mut msg: MessageAddr){
    if msg.msg.msg_data.len() < 8 {
        return;
    }
    let x = msg.msg.msg_data.get_i32();
    let y = msg.msg.msg_data.get_i32();
    let coord = Coord::new(x, y);
    println!("map req: {:?}", coord);
    println!("end map req parse");
}

pub fn parse_obj_ack (mut msg: MessageAddr) {

}


pub fn close_sess (addr: SocketAddr) {

}

pub struct Relative {
    seq: u16,
    msg_type: u8,
}

#[derive(Debug)]
pub struct SessionReq {
    seq: u16,
    protocol: String,
    pver: u16,
    username: String,
    cookie_len: u16,
    cookie: [u8; 32],
    //args
}

pub struct ObjAck {
    id: u64,
    frame: i32,
    received: u64,
    sent: u64,
}



pub async fn run_ipv6(pool: &SqlitePool, global: Arc<global>) -> Result<(), io::Error> {

    let socket = Arc::new(UdpSocket::bind("::1:1870").await?);
    //let socket = UdpSocket::bind("::1:1870").await?;
    let mut buf = [0u8; MTU_SIZE];
    loop {
        let (size, src) = socket.recv_from(&mut buf).await?;
        println!("Bytes len: {:?} from {:?}", size, src);
        println!("received: {:?}", &buf[0..size]);
        //println!("as string: {}", to_string_ascii(&buf));
        //println!()
        // process(&buf[0..size], src, &socket, &global);
        let sock2 = socket.clone();
        let glob2 = global.clone();
        tokio::spawn(async move {
            process(&buf[0..size], src, &sock2, &glob2).await;
        });
        //let len = socket.send_to(&buf[..size], &src).await?;
    }
}