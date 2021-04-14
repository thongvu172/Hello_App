pub mod SecureSocket {

    // Imports
    use tokio::net::*;
    use std::ops::{FnMut};
    use openssl::{ symm::Mode
                 , aes::*
                 , rsa::{Rsa, Padding},pkey::Private};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::str;

   
    pub struct SecureSocket <'a>
        { socket      : &'a TcpStream
        , aes_encrypt : Box::<dyn FnMut(&[u8]) -> Vec<u8>>
        , aes_decrypt : Box::<dyn FnMut(&[u8]) -> Vec<u8>> }

    impl <'a> SecureSocket<'a> {
        pub async fn receve(&self) -> Option<String> {
            let mut buf = CLEARBUFF;
            self.socket.read(&mut buf);
            str::from_utf8(
                &self.aes_decrypt(buf)).ok().map(|x| String::from(x))
        }
    }

    pub async fn new<'a> ( un_secure : &'a mut TcpStream
                    , isClient  : bool
                    , rsa_info  : Option<Rsa<Private>>) 
                    -> Option<SecureSocket<'a>> {
        let mut buf = CLEARBUFF;
        if isClient {
        // 1. Receive RSA public key
        // 2. Turn RSA key into cypher
        // 3. Generate AES key
        // 4. Send AES key
        // 5. Set AES vars
        // TODO
        Some(SecureSocket
            { socket      : un_secure
            , aes_encrypt : Box::new(|x| x.to_vec())
            , aes_decrypt : Box::new(|x| x.to_vec()) })
        } else {
        // 1. Generate public/private key pair
            let rsa = match rsa_info {
                Some(x) => if x.size() == KEYSIZE {
                                x
                            } else
                            {
                                return None
                            },
                None    => Rsa::generate(KEYSIZE).ok()?
            };
        // 2. Send RSA public key
            let public_key = rsa.public_key_to_der().ok()?; 
            un_secure.write(&public_key).await.ok()?;
        // 3. Receive encrypted AES key
            un_secure.read(&mut buf);
        // 4. Decrypt AES key
            let mut buf_ = CLEARBUFF;
            rsa.private_decrypt( &buf
                               , &mut buf_
                               , Padding::PKCS1).ok()?;
        // 5. Set AES vars
            let aesKey_en = AesKey::new_encrypt(&buf_).ok()?;
            let aesKey_de = AesKey::new_decrypt(&buf_).ok()?;
            Some(SecureSocket
                { socket      : un_secure
                , aes_encrypt : 
                    Box::new(move |x| {
                        let mut buf = CLEARBUFF;
                        let mut iv  = CLEARBUFF;
                        aes_ige( x
                               , &mut buf
                               , &aesKey_en
                               , &mut iv
                               , Mode::Encrypt);
                        buf.to_vec()})
                , aes_decrypt : 
                    Box::new(move |x| {
                        let mut buf = CLEARBUFF;
                        let mut iv  = CLEARBUFF;
                        aes_ige( x
                               , &mut buf
                               , &aesKey_de
                               , &mut iv
                               , Mode::Decrypt);
                        buf.to_vec()})
                })
        }
    }
}
