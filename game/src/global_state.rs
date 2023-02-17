use std::fmt::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use dashmap::DashMap;
use crate::character::Character;
use crate::coord::Coord;
use crate::account::Account;
use crate::map::Map;
use crate::AI::AI_Controller;
use crate::session::{Session, SessionError};
use sqlx::{Connection, SqliteConnection, SqlitePool};
use tokio::net::UdpSocket;
use crate::message::{Message, MessageAddr};


pub struct global {
    //character can exist without a session
    sessions: DashMap<SocketAddr, Session>,
    characters: DashMap<Session, Arc<Character>>,
    maps: DashMap<Coord, Map>,
    ai_objs: Vec<AI_Controller>,
    db: SqlitePool,
    pub sock_v4: Arc<UdpSocket>,
    pub sock_v6: Arc<UdpSocket>,
}

impl global {
    pub async fn new() -> Self {
        Self {
            sessions: DashMap::new(),
            characters: DashMap::new(),
            maps: DashMap::new(),
            ai_objs: Vec::new(),
            db: SqlitePool::connect("sqlite:../db/users.db").await.unwrap(),
            sock_v4: Arc::new(UdpSocket::bind("127.0.0.1:1870").await.unwrap()),
            sock_v6: Arc::new(UdpSocket::bind("::1:1870").await.unwrap()),
        }
    }

    pub async fn add_session(&self, addr: SocketAddr, session: Session) -> SessionError {
        if self.sessions.contains_key(&addr) {
            return SessionError::Busy;
        }
        self.sessions.insert(addr, session);
        return SessionError::Success;
    }

    pub fn close_session(&self, addr: SocketAddr) -> Result<u8, Error> {
        if self.sessions.contains_key(&addr) {
            let sess = self.sessions.get(&addr);
            if let Some(mut sess) = sess {
                let s= sess.value();

            }
            self.sessions.remove(&addr);
            return Ok(0);
        }
        else {
            return Err(Error);
        }
    }

    pub fn receive_msg(&self, msg: MessageAddr) {
        let session = self.sessions.get(&msg.socket_addr);
        if let Some(mut session) = session {
            let s: &Session = session.value();
            //s.receive_msg(msg.msg);
        }
    }

    pub async fn send_msg(&self, msg: MessageAddr){
        if msg.socket_addr.is_ipv4() {
            let res = self.sock_v4.send_to(&msg.msg.ser(), msg.socket_addr).await;
        } else {
            let res = self.sock_v6.send_to(&msg.msg.ser(), msg.socket_addr).await;
        }
    }
}