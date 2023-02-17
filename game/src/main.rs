extern crate core;

use argh::FromArgs;
use std::io::{self/* BufReader, BufWriter, Read, Write*/};
use std::net::ToSocketAddrs;
use std::path::{Path, PathBuf};

use std::sync::{Arc, Mutex};
use tokio::io::{copy, sink, split, AsyncWriteExt, AsyncReadExt, AsyncRead};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::TlsAcceptor;
use bytes::{Buf, BytesMut};
use std::fmt::Debug;
use sqlx::{Connection, SqliteConnection, SqlitePool};
use crate::auth::{check_passhash, get_user};

use crate::game_server::{run_ipv4, run_ipv6};


mod auth;
mod msg_util;
mod game_server;
mod global_state;
mod character;
mod coord;
mod account;
mod map;
mod AI;
mod session;
mod object;
mod message;
mod char_create;
mod reliable;
mod objack;
mod mapdata;
mod objdelta;
mod rmessage;


/// Tokio Rustls server example
#[derive(FromArgs)]
pub struct Options {
    /// bind addr
    #[argh(positional)]
    addr: String,

    /// cert file
    #[argh(option, short = 'c')]
    cert: PathBuf,

    /// key file
    #[argh(option, short = 'k')]
    key: PathBuf,

    /// echo mode
    #[argh(switch, short = 'e')]
    echo_mode: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {

    let pool = SqlitePool::connect("sqlite:../db/users.db").await.unwrap();
    let a = "admin".to_string();
    let b = get_user(a, &pool).await.unwrap();
    println!("{:?}", b);

    //in memory database
    //https://www.sqlite.org/inmemorydb.html
    //let conn = SqliteConnection::connect("sqlite::memory:").await?;

    let global = Arc::new(global_state::global::new().await);

    let game_pool = pool.clone();
    let global2 = global.clone();
    let game_server_handle = tokio::spawn(
        async move {
            // let fut = run_ipv4().await?;
            let game_fut = run_ipv4(&game_pool, global2).await;
        }
    );
    let game_pool = pool.clone();
    let global3 = global.clone();
    let game_server_handle_ipv6 = tokio::spawn(
        async move {
            // let fut = run_ipv4().await?;
            let game_fut = run_ipv6(&game_pool, global3).await;
        }
    );
    loop {

    }

}