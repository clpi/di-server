pub mod config;
pub mod state;
pub mod tcp;
pub mod udp;

use dpool::{PoolCreationError, ThreadPool};
use dargs::Args;
use dhttp::{Method, HttpRun};
use std::{
    convert::TryFrom,
    fs::read_to_string,
    io::{self, prelude::*},
    net::{self, Ipv4Addr, Shutdown, Incoming, UdpSocket, TcpListener, TcpStream},
    collections::HashMap,
    sync::Mutex,
    thread, time::Duration
};
use super::{
    route::{Route, Router},
    request::Request,
    response::Response,
};

lazy_static::lazy_static!{
    static ref IN_MEM: Mutex<HashMap<String, String>> =
        Mutex::new(HashMap::new());
}

static LOGGER: ServerLogger = ServerLogger;

pub struct ServerLogger;

impl log::Log for ServerLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[derive(Debug, Clone)]
pub struct Server {
    multicast: Option<Ipv4Addr>,
    address: String,
    debug: bool,
    router: Router,
}

impl Server {

    pub fn new() -> Self {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(log::LevelFilter::Info))
            .expect("Could not set logger");
        log::debug!("Logging initiated");
        Self::try_from(Args::get()).expect("Could not parse args to TCP server")
    }

    pub fn run(&mut self, n_thr: Option<usize>) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        println!("Server listening: {}{}", "http://", self.address);
        let pool = ThreadPool::new(n_thr)
            .expect("Could not establish thread pool");
        let srv = listener.incoming();
        for (n, stream) in srv.enumerate() {
            match stream {
                Err(err) => {
                    eprintln!("Error reading req {}: {}", n, err);
                    return Err(err);
                },
                Ok(stream) => {
                    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
                    pool.execute(|| match Self::handle_conn(stream) {
                        Ok(res) => println!("{}: {}", res, "Handled stream"),
                        Err(_) => eprintln!("Could not handle stream"),
                    });
                },
            }
        }
        Ok(())
    }

    fn handle_conn(mut stream: TcpStream) -> io::Result<String> {
        println!("Connected to {}", stream.local_addr()?);
        let mut buf = [0; 1024];
        stream.read(&mut buf)?;
        let req = String::from_utf8_lossy(&buf[..]);
        println!("Request: {}", req);
        if let Some(req_line) = req.lines().next() {
            match Self::parse_req_line(req_line) {
                Ok(req) => println!("Request: {:?}", req),
                Err(err) => eprintln!("Bad request: {}", err),
            }
        }
        Self::process_req(stream, buf)
    }

    pub fn process_req(mut stream: TcpStream, buf: [u8; 1024]) -> io::Result<String> {
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
        Ok(res)
    }

    pub fn parse_req_line(line: &str)
        -> Result<Request<String>, Box<dyn std::error::Error>> {
        let (mut route, mut method, mut version):
            (Route, dhttp::Method, dhttp::Version)
             = (Route::default(), Method::default(),
                 dhttp::Version::default());
        while let Some((idx, line)) = line.split(" ").enumerate().next() {
            let res = match (idx, line) {
                (0, method_str) => {
                    method = Method::try_from(method_str)?;
                },
                (1, route_str) => {
                    route = Route::try_from(route_str.to_string())
                        .expect("Could not parse route");
                },
                (2, vers_str) => {
                    version = dhttp::Version::try_from(vers_str.to_string())
                        .expect("Could not parse HTTP version");
                },
                (_, _) => {},
            };
        }
        Ok(Request {
            headers: Vec::new(),
            method,
            route,
            http_version: version,
            body: String::new(),
        })
    }

    pub fn parse_method(inp: &str) -> String {
        String::new()
    }

    pub fn method(word: &str) -> Result<Method, Box<dyn std::error::Error>> {
        Ok(Method::try_from(word)?)
    }

    pub fn shutdown(cmd: &str) -> Result<(), io::Error> {
        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {

    }
}

impl TryFrom<Args> for Server {
    type Error = net::AddrParseError;
    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Ok(Self {
            router: Router::default(),
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
