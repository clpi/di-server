use std::io;
use dsrv::server::Server;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "user_auth=info,dsrv=info cargo run");
    Server::new().run(None)
}

