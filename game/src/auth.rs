use std::error::Error;
//use::getrandom::getrandom;
use tokio::io::{copy, sink, split, AsyncWriteExt, AsyncReadExt, self};
use crate::auth;
use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{FromRow, Row};

pub struct UserHash {
    acct_name: String,
    pass_hash: [u8; 32]
}

pub fn gen_new_auth_cookie() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    //TODO return buf1

    let buf2: [u8; 32] = [3u8; 32];
    Ok(buf2)
    //TODO save to database
}

pub async fn save_auth_token(user_name: String, token: [u8; 32], pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let token = token.to_vec();
    let user_id: i64 = get_user_id(user_name, pool).await?;
    let mut conn = pool.acquire().await?;
    // Insert the task, then obtain the ID of this row
    let token_id = sqlx::query!(
        r#"
INSERT INTO tokens ( user_id, token )
VALUES ( ?1, ?2 )
        "#,
        user_id,
        token
    )
        .execute(&mut conn)
        .await?
        .last_insert_rowid();
    Ok(token_id)
}

pub async fn msg_auth_success (account: &mut Vec<u8>) ->  Result<Vec<u8>, io::Error >{
    if account.is_empty(){
        panic!("account must not be empty")
    }
    let mut reply_msg: Vec<u8> = Vec::new();
    reply_msg.write(b"ok\0").await?;
    reply_msg.write(account).await?;
    reply_msg.write(b"\0").await?;
    reply_msg.write(&gen_new_auth_cookie().unwrap()).await?;
    Ok(reply_msg)
}

pub async fn msg_send_cookie () -> Result<Vec<u8>, io::Error>{
    let mut reply_msg: Vec<u8> = Vec::new();
    reply_msg.write(b"ok\0").await?;
    reply_msg.write(&gen_new_auth_cookie().unwrap()).await?;
    //TODO save cookie to db
    Ok(reply_msg)
}

pub async fn msg_auth_fail () -> Vec<u8> {
    let mut reply_msg: Vec<u8> = Vec::new();
    reply_msg.write(b"no\0Invalid username or password\0").await.unwrap();
    reply_msg
}

// #[derive(Debug, FromRow)]
// pub struct User {
//     user_id: i64,
//     user_name: String,
//     user_email: String,
//     user_pass_hash: Vec<u8>,
// }
#[derive(Debug, FromRow)]
pub struct User {
    user_id: i64,
    user_name: String,
    user_email: String,
    user_pass_hash: Vec<u8>,
}

pub struct TokenInfo {
    id: [u8; 16],
    description: String,
}

pub async fn get_user(user_name: String, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    let select_query = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_name = $1")
        .bind(user_name);
    let user: Result<User, sqlx::Error> = select_query.fetch_one(pool).await;
    return user;
}

pub async fn get_user_tokens(user_name: String, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    let select_query = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_name = $1")
        .bind(user_name);
    let user: Result<User, sqlx::Error> = select_query.fetch_one(pool).await;
    return user;
}

pub async fn get_hash(user_name: String, pool: &SqlitePool) -> Result<Vec<u8>, sqlx::Error> {
    let select_query = sqlx::query("SELECT user_pass_hash FROM users WHERE user_name = $1")
        .bind(user_name);
    //fetches the first column of the first row
    //passes up error if there is no row
    return Ok(select_query.fetch_one(pool).await?.get(0));
}

pub async fn get_user_id(user_name: String, pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let select_query = sqlx::query("SELECT user_id FROM users WHERE user_name = $1")
        .bind(user_name);
    //fetches the first column of the first row
    //passes up error if there is no row
    return Ok(select_query.fetch_one(pool).await?.get(0));
}

pub async fn check_passhash(user_hash: [u8; 32],  user_name: String, pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let user = match get_user(user_name, pool).await {
        Ok(u) => u,
        Err(e) => return Ok(false),
    };

    return if user.user_pass_hash == user_hash {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn check_token(token: [u8; 32],  user_name: String, pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let user = match get_user_tokens(user_name, pool).await {
        Ok(u) => u,
        Err(e) => return Ok(false),
    };

    return if user.user_pass_hash == token {
        Ok(true)
    } else {
        Ok(false)
    }
}