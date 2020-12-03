pub mod config;

use dargs::Args;
use std::{
    convert::TryFrom,
    io,
    net::{self,  UdpSocket, Ipv4Addr},
};

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
