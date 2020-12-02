pub mod http;
pub mod cli;
pub mod pool;
pub mod parse;
pub mod request;
pub mod response;
pub mod server;
pub mod client;

pub use server::{Server, UdpServer};
pub use cli::Args;
pub use pool::ThreadPool;
pub use request::Request;
pub use response::{Response, HttpResponse};

#[derive(Debug, Clone)]
pub enum Protocol {
    TCP, UDP,
}

impl From<&str> for Protocol {
    fn from(kind: &str) -> Self {
        match kind {
            "tcp" | "TCP" => Protocol::TCP,
            "udp" | "UDP" => Protocol::UDP,
            _ => Protocol::TCP,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HttpRun {
    Server(Protocol),
    Client(Protocol)
}
