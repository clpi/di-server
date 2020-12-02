use crate::{
    pool::ThreadPool,
    cli::Args,
    http::Method,
};
use std::{
    convert::TryFrom,
    fs::read_to_string,
    io::{self, prelude::*},
    net::{TcpListener, TcpStream, SocketAddrV4},
    thread, time::Duration
};

#[derive(Debug)]
pub struct Server {
    address: SocketAddrV4,
    debug: bool,
}

impl Server {

    pub fn new() -> Self {
        Self::from(Args::get())
    }

    pub fn run(self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        log::info!("Server listening: {}{}", "http://", self.address);
        let pool = ThreadPool::new(4).unwrap();
        for stream in listener.incoming() {
            match stream {
                Err(err) => { eprintln!("Error reading: {}", err); return Err(err) },
                Ok(stream) => {
                    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
                    pool.execute(|| match Self::handle_conn(stream) {
                        Ok(_) => log::info!("{}", "Handled stream"),
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
        log::info!("Request: {}", req);
        if let Some(req_line) = req.lines().next() {
            match Self::parse_req(req_line) {
                Ok(req) => log::info!("Request: {}", req),
                Err(err) => log::error!("Bad request: {}", err),
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
        let method = line.split(" ").next().ok_or("Method not specified")?;
        let method = Method::try_from(method)?;
        Ok(line.to_string())
    }

    pub fn method(word: &str) -> Result<Method, Box<dyn std::error::Error>> {
        Ok(Method::try_from(word)?)
    }
}

impl From<Args> for Server {
    fn from(args: Args) -> Self {
        Self {
            debug: args.debug,
            address: args.get_addr(),
        }
    }
}

pub enum HttpRequest {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    NONE,
}


