use std::fmt::Error;
use std::io;
use std::io::{ Read};
use std::str::Utf8Error;
//use std::io::Write;
use bytes::{Buf, BytesMut};
use tokio::io::{copy, sink, split, AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio_rustls::TlsStream;
use crate::auth;
use nom::{Needed, AsBytes};



pub fn add_msg_length_header(vec: &mut Vec<u8>){
    if vec.len() > u16::MAX as usize - 2 {
        panic!("message length cannot be greater than {}", u16::MAX - 2);
        //TODO return error
    }
    let length_bytes: [u8; {2}] = (vec.len() as u16).to_be_bytes();
    vec.insert(0, length_bytes[1]);
    vec.insert(0, length_bytes[0]);

    // let mut new_vec: Vec<u8> = Vec::new();
    // new_vec.put_u16(vec.len() as u16);
    // new_vec.append(vec);
    // return;

}

/// Gets first two bytes and returns the length of the message
/// # Panics
/// if the message is not long enough
/// # Errors
/// if the message is not long enough
pub fn remove_length_header(read_buffer: &mut BytesMut) -> Result< u16, Error> {
    //check if there are enough bytes
    if read_buffer.len() < 3 {
        return Err(Error);
    }
    let length: u16 = read_buffer.get_u16();
    Ok(length)
}


/// Gets the next utf8 string from the message
/// # Panics
/// if the message is not valid utf8
/// # Errors
/// if the message is not valid utf8
pub fn get_next_string_utf8(read_buffer: &mut BytesMut) -> Result<String, Utf8Error> {
    let mut vec: Vec<u8> = Vec::new();
    for _ in 0..read_buffer.len() {
        let b = read_buffer.get_u8();
        if b == 0 {
            break;
        }
        else { vec.push(b); }
    }
    let str = std::str::from_utf8(&vec)?;
    Ok(String::from(str))
}

pub fn parse_nul_terminated_ascii(input: &[u8]) -> nom::IResult<&[u8], &str> {
    let bytes = input.as_bytes();
    let mut nul_offset = None;
    for i in 0..input.len() {
        if bytes[i] & 0b_1000_0000 != 0 {
            return Err(nom::Err::Error(nom::error::Error {
                input: &bytes[i..],
                code: nom::error::ErrorKind::IsNot,
            }))
        }
        if bytes[i] == b'\0' {
            nul_offset = Some(i);
            break;
        }
    }
    let nul_offset = nul_offset.ok_or(nom::Err::Incomplete(Needed::Unknown))?;
    let next = nul_offset + 1;
    let input = &bytes[next..];
    let s = std::str::from_utf8(&bytes[..nul_offset]).unwrap();
    Ok((input, s))
}

/// Returns a ascii String representation of the buffer
pub fn to_string_ascii(read_buffer: &BytesMut) -> String{
    let mut string_buffer = String::new();
    for byte in read_buffer.bytes() {
        if byte.as_ref().unwrap() >= &0x20 && byte.as_ref().unwrap() <= &0x7E {
            string_buffer.push(byte.unwrap() as char);
        } else {
            string_buffer.push_str(&format!("\\x{:x}", byte.unwrap()));
        }
    }
    return string_buffer;
}

/// Returns a ascii String representation of the buffer
pub fn to_string_ascii_2(read_buffer: &[u8]) -> String{
    let mut string_buffer = String::new();
    for byte in read_buffer {
        if byte >= &0x20 && byte <= &0x7E {
            string_buffer.push(*byte as char);
        } else {
            string_buffer.push_str(&format!("\\x{:x}", byte));
        }
    }
    return string_buffer;
}


pub fn get_next_32_bytes(read_buffer: &mut BytesMut) -> Result<[u8; 32], Error> {
    //check if there are enough bytes
    if read_buffer.len() < 32 {
        return Err(Error);
    }
    let mut bytes: [u8; 32] = [0u8; 32];
    for i in 0..32 {
        bytes[i] = read_buffer.get_u8();
    }
    Ok(bytes)
}

pub fn get_next_16_bytes(read_buffer: &mut BytesMut) -> Result<[u8; 16], Error> {
    //check if there are enough bytes
    if read_buffer.len() < 16 {
        return Err(Error);
    }
    let mut bytes: [u8; 16] = [0u8; 16];
    for i in 0..16 {
        bytes[i] = read_buffer.get_u8();
    }
    Ok(bytes)
}

//gets the next int64
pub fn get_next_i64 (buf: &mut BytesMut) -> Result<i64, Error> {
    if buf.len() < 8 {
        return Err(Error);
    }
    let i = buf.get_i64();
    Ok(i)
}

// pub async fn send_message(mut vec: Vec<u8>, writer: &mut WriteHalf<TlsStream<TcpStream>>) -> io::Result<()>{
//     add_msg_length_header(&mut vec);
//     writer.write_all(&vec[..vec.len()]).await?;
//     writer.flush().await?;
//     Ok(()) as io::Result<()>
// }
