use std::io;
use dsrv::tcp::Server;

fn main() -> io::Result<()> {
    Server::new().run()
}
