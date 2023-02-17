# MMO Server

WIP writing an MMO server emulator in async Rust with tokio. Also my first big Rust project. 

Currently reverse engineering the custom network and data serializtion protocol. 

### Folders
/auth the auth service that runs on TCP port 1871

/game the game server that runs on UDP port 1870

## Running

Requires the generation of a self signed certificate with an alt name of the host. See /keygen

run --package auth --bin auth -- 127.0.0.1:1871 --cert newcert.pem --key newkey.pem

Environment variables:
DATABASE_URL=sqlite:./db/users.db


run --package game --bin game

Environment variables:
DATABASE_URL=sqlite:./db/users.db
