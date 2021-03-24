use tokio::net::TcpListener;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use openssl::rsa::{Rsa, Padding};
// use std::convert::TryInto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const SIZE : usize = 2048;
    // Cryptography setup
    let rsa = Rsa::generate(2048).unwrap();
    //Networking Setup
    let listener = TcpListener::bind("0.0.0.0:6969").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let rsa = rsa.clone();

        tokio::spawn(async move {
            let mut buf : [u8;SIZE] = [0; SIZE];

            // 1. @TODO Make connection encrypted
            // 1.1 send public key
            let public_key = rsa.public_key_to_pem().ok()?; //.try_into().ok()?;
            socket.write(&public_key).await.ok()?;
            // 1.2 receive AES key
            // @TODO Send "Hello" through  encrypted tunnel  privet 
            socket.write("HELLO Γ".as_bytes()).await.ok()?;
            // @TODO Receive "GOODBYE" through encrypted tunnel
            socket.read(&mut buf).await.ok()?;
            let message : String = String::from(str::from_utf8(&buf).ok()?);
            if message == "GOODBYE Γ" {
                println!("Successful Conversation");
            }
            else {
                println!("Unsuccessful Conversation; got({})",message);
            }
            // @TODO Close Connection (when socket leaves scope)
            Some(()) 
        });
    }
}
