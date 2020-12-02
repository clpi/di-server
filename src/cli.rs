use std::{env, net::SocketAddrV4};

#[derive(Debug, Clone)]
pub struct Args {
    pub host: String,
    pub port: String,
    pub debug: bool,
    protocol: Protocol,
    help: bool,
    version: bool,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    TCP, UDP
}

impl Args {

    pub fn get() -> Args {
        let args: Vec<String> = env::args().collect();
        Self::process_args(args)
    }

    fn process_args(args: Vec<String>) -> Self {
        let mut out: Args = Args::default();
        let mut flagged: Option<&str> = None;
        args[1..].iter().zip(1..args.len()+1).for_each(|(arg, n)| {
            if arg.starts_with("--") {
                let flag = arg.split_at(2).1;
                match flag {
                    "host" => {
                        if let Some(val) = args.get(n+1) {
                            flagged = Some(flag);
                            out.host = val.to_string(); //TODO validate
                            println!("Host: {}", val)
                        }
                    },
                    "port" => {
                        if let Some(val) = args.get(n+1) {
                            flagged = Some(flag);
                            out.port = val.to_string();
                            println!("Port: {}", val);
                        }
                    },
                    "debug" => {
                        out.debug = true;
                        println!("Debug is on");
                    },
                    "version" => {
                        out.version = true;
                        println!("Version is on");
                    },
                    "help" => {
                        out.help = true;
                        println!("Help is on")
                    },
                    "udp" => {
                        out.protocol = Protocol::UDP;
                        println!("Protocol set to UDP")
                    },
                    "tcp" => println!("TCP is default"),
                   _ => println!("No valid flag selected: {}", flag),
                }
            } else if arg.starts_with("-") {
                let flag = arg.split_at(1).1;
                match flag {
                    "h" => {
                        if let Some(val) = args.get(n+1) {
                            flagged = Some(flag);
                            out.host = val.to_string();
                            println!("Host: {}", val);
                        }
                    },
                    "p" => {
                        if let Some(val) = args.get(n+1) {
                            flagged = Some(flag);
                            out.port = val.to_string();
                            println!("Port: {}", val);
                        }
                    },
                    "d" => {
                        out.debug = true;
                        println!("Debug is on");
                    },
                    "v" => {
                        out.version = true;
                        println!("Version is on");
                    },
                    "u" => {
                        out.protocol = Protocol::UDP;
                        println!("Protocol set to UDP");
                    },
                    "t" => println!("TCP is default"),
                    _ => {
                        println!("No valid option selected: {}", flag)
                    },
                }
            } else if flagged.is_none() {
                let cmd = arg.as_str();
                match cmd {
                    "help" => {
                        out.help = true;
                        println!("Help subcmd is on")
                    },
                    "version" => {
                        out.version = true;
                        println!("Version subcmd is on")
                    },
                    "debug" => {
                        out.debug = true;
                        println!("Debug subcmd is on")
                    },
                    "udp" => {
                        out.protocol = Protocol::UDP;
                        println!("Debug subcmd is on")
                    },
                    _ => println!("Invalid subcmd: {}", cmd),
                }
            }
        });
        out
    }

    pub fn get_addr(self) -> SocketAddrV4 {
        self.get_addr_string()
            .parse()
            .expect("Could not parse host and port into socket address")

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
            protocol: Protocol::TCP,
            host: String::from("127.0.0.1"),
            port: String::from("8080"),
        }
    }
}