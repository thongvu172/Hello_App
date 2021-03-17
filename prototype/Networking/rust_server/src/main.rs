use tokio::net::TcpListener;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::rngs::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const SIZE : usize = 2048;
    // Cryptography setup
    let mut rng = OsRng;
    let private_key = RSAPrivateKey::new(&mut rng, SIZE).expect("failed to generate a key");
    //Networking Setup
    let listener = TcpListener::bind("0.0.0.0:6969").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; SIZE];

            // 1. @TODO Make connection encrypted
            // 1.1 send public key
            socket.write(private_key.to_public_key()).await.ok()?
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
