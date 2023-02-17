extern crate core;

use argh::FromArgs;
use rustls_pemfile::{certs, pkcs8_private_keys, /*rsa_private_keys*/};
use std::fs::File;
use std::io::{self, BufReader, /*BufWriter,*/ Read/*, Write*/};
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
use crate::auth_service::run_auth_service;

use crate::auth_message::{get_next_32_bytes, to_string_ascii, get_next_string_utf8};

mod auth;
mod auth_message;
mod auth_service;


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

}

#[tokio::main]
async fn main() -> io::Result<()> {

    let pool = SqlitePool::connect("sqlite:../db/users.db").await.unwrap();
    let a = "admin".to_string();
    let b = get_user(a, &pool).await.unwrap();
    println!("{:?}", b);

    let options: Options = argh::from_env();
    let auth_service = tokio::spawn(
        async move {
            // let auth_fut = run_auth_service(options, &pool.clone()).await?;
            let auth_fut = run_auth_service(options, &pool).await;
        }
    );

    loop {

    }

}