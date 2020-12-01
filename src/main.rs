pub mod args;
pub mod pool;
pub mod request;
pub mod response;
pub mod server;

use std::io;
use server::Server;

fn main() -> io::Result<()> {
    Server::new().run()
}



// impl IntoIterator for HttpRequest {
//     fn into_iter(self) -> dyn IntoIterator<IntoIter= dyn Into<&'static str>+Sized+'static,Item=&'static str + 'static> {
//         vec!["GET", "POST", "PUT", "DELETE", "HEAD"].into_iter()
//     }
// }

