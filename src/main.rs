use core::str;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:5354")?;
    println!("DNS SERVER RUNNING ON PORT 5354");

    let mut buffer = [0; 512];

    loop {
        let (len, src) = socket.recv_from(&mut buffer)?;
        match str::from_utf8(&buffer[..len]) {
            Ok(msg) => {
                println!("BUFFER : {}", msg);
            }
            Err(e) => {
                println!("{:?}", &buffer[..len]);
                println!("ERROR : {}", e);
            }
        }
        println!("SRC : {}", src);

        let res = "Message Recieved!";
        socket.send_to(res.as_bytes(), &src)?;        
    }
}
