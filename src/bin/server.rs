use std::io;
use dsrv::server::Server;

fn main() -> io::Result<()> {
    Server::new().run()
}
