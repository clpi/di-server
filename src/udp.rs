use crate::cli::Args;
use std::{
    convert::TryFrom,
    io,
    net::{self,  UdpSocket, Ipv4Addr},
    thread
};

#[derive(Debug, Default)]
pub struct UdpServer {
    multicast: Option<Ipv4Addr>,
    address: String,
    debug: bool,
}

impl UdpServer {

    pub fn from_args() -> Self {
        Self::try_from(Args::get()).expect("Could not create UDP servere from args")
    }

    pub fn bind_wildcard() -> Self {
        Self { address: "0.0.0.0".into(), ..Self::default() }
    }

    pub fn run(&self) -> io::Result<()> {
        let socket = UdpSocket::bind(self.address.as_str())?;
        let wildcard = "0.0.0.0".parse::<Ipv4Addr>().unwrap();
        if let Some(mc) = &self.multicast {
            socket.join_multicast_v4(mc, &wildcard)
                .expect("Could not join multicast");
        }
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

impl TryFrom<Args> for UdpServer {
    type Error = net::AddrParseError;
    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Ok(Self {
            debug: args.debug,
            address: args.clone().get_addr_string(),
            multicast: Some(args.get_multicast()?),
        })
    }
}

impl TryFrom<Args> for UdpClient {
    type Error = net::AddrParseError;
    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Ok(Self {
            address: args.clone().get_addr_string(),
            multicast: Some(args.get_multicast()?),
        })
    }
}

pub struct UdpClient {
    address: String,
    multicast: Option<Ipv4Addr>
}

impl UdpClient {

    pub fn from_args() -> Self {
        Self::try_from(Args::get()).expect("Could not get client from args")
    }

    pub fn connect(self, msg: Option<String>) -> io::Result<()> {
        let socket = UdpSocket::bind(self.address.as_str())?;
        let wildcard = "0.0.0.0".parse::<Ipv4Addr>().unwrap();
        if let Some(mc) = &self.multicast {
            socket.join_multicast_v4(mc, &wildcard)
                .expect("Could not join multicast");
        }
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
