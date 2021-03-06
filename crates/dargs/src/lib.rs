#[macro_use] extern crate quote;
pub mod derive;
pub mod args;

use dhttp::{HttpRun, Protocol};
use std::{env, net::{self, Ipv4Addr, SocketAddrV4}, fmt};

//TODO Generalize this for any sort of input args
/// Represents CLI arguments parsed into a struct for running the server
#[derive(Debug, Clone)]
pub struct Args {
    pub host: String,
    pub port: String,
    pub debug: bool,
    pub multicast: Option<String>,
    run: HttpRun,
    help: bool,
    version: bool,
    pub async_run: bool,
}


impl Args {

    pub fn get() -> Args {
        let args: Vec<String> = env::args().collect();
        Self::process_args(args)
    }

    fn process_args(args: Vec<String>) -> Self {
        let mut flagged: Option<&str> = None;
        let mut out = Args::default();
        args[1..].iter().zip(1..args.len()+1).fold(&mut out, |out, (arg, n)| {
            let bool_flag = match arg.clone().as_str() {
                "--debug" | "-d" => {
                    out.debug = true;
                    println!("Debug is on");
                    true
                },
                "--version" | "-v" => {
                    out.version = true;
                    println!("Version is on");
                    true
                },
                "--help" | "help" => {
                    out.help = true;
                    println!("Help is on");
                    true
                },
                "--async" | "-a" => {
                    out.async_run = true;
                    println!("Async is on");
                    true
                }
               _ => false,
            };
            let val = args.get(n+1);
            if val.is_some() && !bool_flag && flagged.is_none() {
                let val = val.unwrap();
                let (lev, input) = match arg[..2].split_at(1) {
                    ("-", "-") => (2, &arg[2..]),
                    ("-", _) => (1, &arg[1..]),
                    _ => (0, arg.as_str())
                };
                flagged = Some(input);
                match (lev, input)  { //TODO struct-ize this match arg
                    (2, "host") | (1, "h")  => {
                        out.host = val.into(); //TODO validate
                        println!("Host: {}", val)
                    },
                    (2, "port") | (1, "p")  => {
                        out.port = val.into();
                        println!("Port: {}", val);
                    },
                    (2, "multicast") | (1, "m")  => {
                        out.multicast = Some(val.into());
                        println!("Multicast: {}", val);
                    },
                    (2, "server") | (1, "s") | (0, "server") => {
                        let protocol = Protocol::from(val.as_str());
                        out.run = HttpRun::Server(protocol.clone());
                        println!("Running {} server", protocol.clone());
                    },
                    (2, "client") | (1, "c") | (0, "client") => {
                        let protocol = Protocol::from(val.as_str());
                        out.run = HttpRun::Client(protocol.clone());
                        println!("Running {} client", protocol);
                    },
                    _ => {
                        eprintln!("Invalid flag or subcmd provided");
                        std::process::exit(1);
                    }
                }
            } else if flagged.is_some() { flagged = None }
            out
        }).to_owned()
    }

    pub fn get_addr(self) -> SocketAddrV4 {
        self.get_addr_string()
            .parse()
            .expect("Could not parse host and port into socket address")
    }

    pub fn get_multicast(self) -> Result<Ipv4Addr, net::AddrParseError> {
        self.multicast.unwrap_or_default()
            .parse::<Ipv4Addr>()
    }

    pub fn get_addr_string(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            debug: false,
            help: false,
            version: false,
            async_run: false,
            run: HttpRun::Server(Protocol::UDP),
            host: String::from("127.0.0.1"),
            port: String::from("8080"),
            multicast: None,
        }
    }
}

// pub trait Arg  {}

// fn impl_args(ast: &syn::DeriveInput) {
//     let name = &ast.ident;
//     quote! {
//         impl Arg for #name {
//         }
//     };
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
