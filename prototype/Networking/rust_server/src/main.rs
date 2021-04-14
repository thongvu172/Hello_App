// mod Lib;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_native_tls::{TlsAcceptor, TlsStream};
use tokio_native_tls::native_tls; 
use std::str;
use std::io::*;
use std::fs::File;


const BUFFSIZE  : usize = 2048;
const CLEARBUFF : [u8;BUFFSIZE] = [0;BUFFSIZE];
 

#[tokio::main]
async fn main() -> Result<()/*, Box<dyn std::error::Error>*/> {
    // Get Crypto info
    // files from command "openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem
    // -certfile chain_certs.pem"
    let (der,pass) = 
        match get_ssl_info() {
            Some(x) => x,
            None    => 
            {
                eprintln!("Failed to get Crypto info. Exiting");
                std::process::exit(1);
            }
        };
    let identity = native_tls::Identity::from_pkcs12(&der, &pass).unwrap();
    let accepter : TlsAcceptor = 
        TlsAcceptor::from(
            native_tls::TlsAcceptor::new(identity).unwrap());
    // Networking Setup
    let listener = TcpListener::bind("0.0.0.0:6969").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        let accepter        = accepter.clone();
        tokio::spawn(async move {
            let mut socket : TlsStream<&mut TcpStream> = accepter.accept(&mut socket).await.ok()?;
            // echo server
            println!("start");
            loop {
                let mut buf = CLEARBUFF;
                // get message
                match socket.read(&mut buf).await {
                    Ok(0)  => break,
                    Err(_) => break,
                    Ok(_)  => ()
                };
                let message : String = 
                    String::from(
                        str::from_utf8(
                            &buf).ok()?);
                println!("Received:{}",message);
                socket.write(&buf).await.ok()?;
            };
            println!("end");
            Some(())
        });
    }
}


fn get_ssl_info() -> Option<(Vec<u8>,String)> {
    const derName : &str = "ssl/identity.pfx";
    const passName : &str = "ssl/pass";
    let mut buf = Vec::new();
    // get der
    let mut reader = File::open(derName).ok()?;
    reader.read_to_end(&mut buf).ok()?;
    let der = buf.clone();
    buf = Vec::new();
    // get pass
    reader = File::open(passName).ok()?;
    reader.read_to_end(&mut buf).ok()?;
    let pass = str::from_utf8(&buf).ok()?;
    // end
    Some((der,pass.trim().to_string()))
}
