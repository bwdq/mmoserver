
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::io::{copy, sink, split, AsyncWriteExt, AsyncReadExt, AsyncRead};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::TlsAcceptor;
use bytes::{Buf, BytesMut};
use std::fmt::Debug;
use std::error::Error;
use sqlx::{Connection, SqliteConnection, SqlitePool};

use crate::auth::{check_passhash, check_token, get_user};
use crate::auth_message::{get_next_32_bytes, to_string_ascii, get_next_string_utf8};
use crate::{auth, auth_message, Options};

pub async fn run_auth_service(options: Options,  pool: &SqlitePool) -> io::Result<()> {

    let addr = options
        .addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;
    let certs = load_certs(&options.cert)?;
    let mut keys = load_keys(&options.key)?;


    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let pool = pool.clone();
        dbg!("new auth peer: {}", peer_addr);
        tokio::spawn(async move {
            if let Err(e) = process_auth(stream, acceptor, pool).await {
                dbg!("failed to process connection; error = {}", e);
            }
        });
    };
}

async fn process_auth(stream: TcpStream, acceptor: TlsAcceptor,
                      pool: SqlitePool) -> Result<(), Box<dyn Error>> {

    let mut stream = acceptor.accept(stream).await?;

    let (mut reader,
        mut writer) = split(stream);
    //create read buffer
    let mut read_buffer = BytesMut::with_capacity(1024);
    //let read_buffer: Buf = Buf::new();

    let mut authed: bool = false;
    loop {
        //read into buffer
        let mut num_bytes = reader.read_buf(&mut read_buffer).await?;

        if num_bytes == 0 {
            println!("connection closed");
            break;
        }

        //handle the message
        println!("buf: {:?}", read_buffer.bytes());
        println!("buf as printable ascii: {}", to_string_ascii(&read_buffer));

        // let header_len = message::remove_length_header(&mut read_buffer).expect(
        //     "Not enough bytes in buffer");

        //if error is returned, then the message is not long enough to be a valid message
        let header_len = auth_message::remove_length_header(&mut read_buffer)?;

        println!("buf no header: {:?}", read_buffer.bytes());
        if (num_bytes as u16) - 2 != header_len {
            dbg!("length mismatch error");
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "length mismatch error")));
        }
        let cmd = get_next_string_utf8(&mut read_buffer).unwrap();
        println!("cmd: {}", cmd);
        println!("buf no cmd: {:?}", read_buffer.bytes());

        //if cmd is pw then get pw
        //cmd("pw", user, phash);
        if cmd == "pw" {
            //get username
            let account_name = get_next_string_utf8(&mut read_buffer).unwrap();
            println!("user: {}", account_name);
            let pass_hash = get_next_32_bytes(&mut read_buffer).unwrap();
            println!("pass_hash: {:?}", pass_hash);
            authed = check_passhash(pass_hash, account_name.clone(), &pool).await.unwrap();
            dbg!("check: {}", authed);
            let mut reply_msg;
            if authed {
                reply_msg = auth::msg_auth_success(
                    &mut account_name.as_bytes().to_vec()).await.unwrap();
            }
            else {
                reply_msg = auth::msg_auth_fail().await;
            }

            //add header
            auth_message::add_msg_length_header(&mut reply_msg);
            println!("reply msg with header{:?}", reply_msg.as_slice());
            //write to socket and send
            writer.write_all(&reply_msg[..reply_msg.len()]).await?;
            writer.flush().await?;
            //read second
            read_buffer.clear();
        }
        //cmd("token", user, token); token 32 bytes
        if cmd == "token" {
            //get username
            let account_name = get_next_string_utf8(&mut read_buffer).unwrap();
            println!("user: {}", account_name);
            let token = get_next_32_bytes(&mut read_buffer).unwrap();
            println!("token: {:?}", token);
            authed = check_token(token, account_name.clone(), &pool).await.unwrap();
            println!("check: {}", authed);
            let mut reply_msg;
            if authed {
                reply_msg = auth::msg_auth_success(
                    &mut account_name.as_bytes().to_vec()).await.unwrap();
            }
            else {
                reply_msg = auth::msg_auth_fail().await;
            }

            //add header
            auth_message::add_msg_length_header(&mut reply_msg);
            println!("reply msg with header{:?}", reply_msg.as_slice());
            //write to socket and send
            writer.write_all(&reply_msg[..reply_msg.len()]).await?;
            writer.flush().await?;
            //read second
            read_buffer.clear();
        }
        //if cmd cookie, if authed return cookie
        if cmd == "cookie" {
            println!("num bytes read: {:?}, content: {:?}", num_bytes, read_buffer);
            //println!("contents of bytesmut buffer: {:?}", bytes_read);
            //write cookie
            //let mut reply_msg2 = auth::msg_send_cookie().await.unwrap();
            let mut reply_msg;
            if authed {
                reply_msg = auth::msg_send_cookie().await.unwrap();
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "not authed"))?;
            }
            auth_message::add_msg_length_header(&mut reply_msg);
            writer.write_all(&reply_msg[..reply_msg.len()]).await?;
            writer.flush().await?;
            read_buffer.clear();
        }
        //cmd("mktoken", info.encode());
        //"id", this.id 16 bytes, random
        //"desc", this.desc string hostname
        if cmd == "mktoken" {
            //8 T_TTOL type object
            //2 T_STRING type string
            //"id\0"
            //14 T_BYTES type bytes
            //16 length
            //0 T_END
            //8 T_TTOL type object
            //2 T_STRING type string
            //"desc\0"
            //8 T_STRING type string
            //hostname\0
            //0 T_END


        }
    }
    Ok(())
}

fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())

}