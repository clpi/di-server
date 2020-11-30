pub mod pool;
pub mod request;
pub mod response;

use pool::ThreadPool;
use std::{
    fs::read_to_string,
    io::{self, prelude::*},
    net::{TcpListener, TcpStream}, thread, time::Duration};

fn main() -> io::Result<()> {
    let addr = "0.0.0.0:7878";
    let listener = TcpListener::bind(addr)?;
    log::info!("Server listening: {}{}", "http://", addr);
    let pool = ThreadPool::new(4).unwrap();
    for stream in listener.incoming().take(2) {
        match stream {
            Ok(stream) => pool.execute(|| {
                match handle_conn(stream) {
                    Ok(_) => log::info!("{}", "Handled stream"),
                    Err(_) => log::error!("Could not handle stream"),
                }
            }),
            Err(err) => log::error!("Error reading: {}", err),
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
        match parse_req(req_line) {
            Ok(req) => log::info!("Request: {}", req),
            Err(err) => log::error!("Bad request: {}", err),
        }
    }

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
    Ok(line.to_string())
}

pub enum HttpRequest {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    NONE,
}


impl From<&str> for HttpRequest {
    fn from(input: &str) -> Self {
        match input {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "HEAD" => Self::HEAD,
            _ => Self::NONE,
        }
    }
}

impl Into<&str> for HttpRequest {
    fn into(self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::HEAD => "HEAD",
            _ => "NONE",
        }
    }
}

// impl IntoIterator for HttpRequest {
//     fn into_iter(self) -> dyn IntoIterator<IntoIter= dyn Into<&'static str>+Sized+'static,Item=&'static str + 'static> {
//         vec!["GET", "POST", "PUT", "DELETE", "HEAD"].into_iter()
//     }
// }

