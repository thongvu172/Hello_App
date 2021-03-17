use tokio::net::TcpListener;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:6969").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // @TODO Make connection encrypted
            // @TODO Send "Hello" through  encrypted tunnel  privet 
            socket.write("HELLOK Γ".as_bytes()).await.ok()?;
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
