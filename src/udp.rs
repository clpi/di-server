use crate::cli::Args;
use std::{
    thread, io, net::{UdpSocket, Ipv4Addr},
};

#[derive(Debug)]
pub struct UdpServer {
    address: String,
    debug: bool,
}

impl UdpServer {

    pub fn from_args() -> Self {
        Self::from(Args::get())
    }

    pub fn run(&self) -> io::Result<()> {
        let socket = UdpSocket::bind(self.address.as_str())?;
        loop {
            let mut buf = [0u8; 1500];
            let udp_socket = socket.try_clone()?;
            match udp_socket.recv_from(&mut buf) {
                Ok((_, src)) => {
                    thread::spawn(move || {
                        println!("Handling connection from {}", src);
                        udp_socket.send_to(&buf, &src)
                    });
                },
                Err(e) => eprintln!("Couldn't receive data: {}", e),
            }
        }
    }
}

impl From<Args> for UdpServer {
    fn from(args: Args) -> Self {
        Self {
            debug: args.debug,
            address: args.get_addr_string(),
        }
    }
}

impl From<Args> for UdpClient {
    fn from(args: Args) -> Self {
        Self { address: args.get_addr_string(), }
    }
}

pub struct UdpClient {
    address: String
}

impl UdpClient {

    pub fn from_args() -> Self {
        Self::from(Args::get())
    }

    pub fn connect(self, msg: Option<String>) -> io::Result<()> {
        let socket = UdpSocket::bind(self.address.as_str())?;
        socket.connect(self.address.as_str())?;
        loop {
            let mut buf = [0u8; 1500];
            let msg = msg.clone().unwrap_or("Hello there!".into());
            socket.send(msg.as_bytes())?;
            socket.recv_from(&mut buf)?;
            print!("{}",  String::from_utf8(buf.to_vec())
                .expect("Could not write buffer as string"));
        }
    }
}
