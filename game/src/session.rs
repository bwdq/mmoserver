use std::net::SocketAddr;
use crate::character::Character;
use crate::coord::Coord;
use std::time::SystemTime;
use crate::message::{Message, MessageType};

const SESSERR_NONE: u8 = 0;
const SESSERR_AUTH: u8 = 1;
const SESSERR_BUSY: u8 = 2;
const SESSERR_CONN: u8 = 3;
const SESSERR_PVER: u8 = 4;
const SESSERR_EXPR: u8 = 5;
const SESSERR_MESG: u8 = 6;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum SessionError {
    Success = SESSERR_NONE,
    Auth = SESSERR_AUTH,
    Busy = SESSERR_BUSY,
    Conn = SESSERR_CONN,
    Pver = SESSERR_PVER,
    Expr = SESSERR_EXPR,
    Mesg = SESSERR_MESG,
    Error = 255,
}

impl SessionError {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Session {
    username: String,
    sock_addr: SocketAddr,
    last_heartbeat: SystemTime,//close after 5000ms
    char: Option<Character>,
    // rx: f
    // tx: flume
    map_requests: Vec<Coord>,
    //rel_msgs: Vec<Message?>,
}

impl Session {

    pub fn new (username: String, sock_addr: SocketAddr) -> Self {
        Self {
            username,
            sock_addr,
            last_heartbeat: SystemTime::now(),
            char: None,
            map_requests: Vec::new(),
        }
    }

    pub fn add_map_request(&mut self, coord: Coord) {
        if !self.map_requests.contains(&coord) {
            self.map_requests.push(coord);
        }
    }

    pub fn remove_map_request(&mut self, coord: Coord) {
        self.map_requests.retain(|&x| x != coord);
    }

    pub fn receive_msg(&mut self, msg: Message) {
        match msg.msg_type {
            MessageType::Beat => {
                self.last_heartbeat = SystemTime::now();
                println!("{:?}", self.last_heartbeat);
            },
            MessageType::MapRequest => {
                println!("Map request: {:?}", msg);
                // let coord = Coord::from_msg(msg);
                // self.add_map_request(coord);
                // println!("Map request: {:?}", coord);
            },
            _ => {
                println!("Unhandled message type: {:?}", msg.msg_type);
            },
        }

    }

    pub fn get_map_requests(&self) -> &Vec<Coord> {
        &self.map_requests
    }

    pub fn beat(&mut self) {
        self.last_heartbeat = SystemTime::now();
    }

    pub fn run(&mut self) {
        // while self.last_heartbeat.elapsed().unwrap().as_millis() < 5000 {
        //     // self.receive_msg();
        // }
        loop {
            //receive channel messages

        }
    }
}


