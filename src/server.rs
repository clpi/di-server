pub mod config;
pub mod state;
pub mod tcp;
pub mod udp;

use dpool::ThreadPool;
use dargs::Args;
use dhttp::{Method, HttpRun};
use std::{
    convert::TryFrom,
    fs::read_to_string,
    io::{self, prelude::*},
    net::{self, Ipv4Addr, UdpSocket, TcpListener, TcpStream},
    thread, time::Duration
};

#[derive(Debug, Clone)]
pub struct Server {
    multicast: Option<Ipv4Addr>,
    address: String,
    debug: bool,
}

impl Server {

    pub fn new() -> Self {
        Self::try_from(Args::get()).expect("Could not parse args to TCP server")
    }

    pub fn run(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        println!("Server listening: {}{}", "http://", self.address);
        let pool = ThreadPool::new(Some(4)).unwrap();
        for stream in listener.incoming() {
            match stream {
                Err(err) => { eprintln!("Error reading: {}", err); return Err(err) },
                Ok(stream) => {
                    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
                    pool.execute(|| match Self::handle_conn(stream) {
                        Ok(_) => println!("{}", "Handled stream"),
                        Err(_) => eprintln!("Could not handle stream"),
                })}
            }
        }
        Ok(())
    }

    fn handle_conn(mut stream: TcpStream) -> io::Result<()> {
        println!("Connected to {}", stream.local_addr()?);
        let mut buf = [0; 1024];
        stream.read(&mut buf)?;
        let req = String::from_utf8_lossy(&buf[..]);
        println!("Request: {}", req);
        if let Some(req_line) = req.lines().next() {
            match Self::parse_req(req_line) {
                Ok(req) => println!("Request: {}", req),
                Err(err) => eprintln!("Bad request: {}", err),
            }
        }
        Self::process_req(stream, buf)?;
        Ok(())
    }

    pub fn process_req(mut stream: TcpStream, buf: [u8; 1024]) -> io::Result<()> {
        let get = b"GET / HTTP/1.1\r\n";
        let post = b"POST / HTTP/1.1\r\n";
        let delete = b"DELETE / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let (status, file) = if buf.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
        } else if buf.starts_with(post) {
            ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
        } else if buf.starts_with(delete) {
            ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
        } else if buf.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "static/404.html")
        };
        let file = read_to_string(file)?;
        let res = format!("{}{}", status, file);
        stream.write(res.as_bytes())?;
        stream.flush()?;

        stream.write(res.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    pub fn parse_req(line: &str) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(line) = line.split(" ").next() {
            let method = Method::try_from(line)?;
        }
        Ok(line.to_string())
    }

    pub fn method(word: &str) -> Result<Method, Box<dyn std::error::Error>> {
        Ok(Method::try_from(word)?)
    }
}

impl TryFrom<Args> for Server {
    type Error = net::AddrParseError;
    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Ok(Self {
            debug: args.debug,
            address: args.clone().get_addr_string(),
            multicast: None,
        })
    }
}

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
            multicast: None,
        })
    }
}
