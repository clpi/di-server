use std::env;

#[derive(Debug, Clone)]
pub struct Args {
    pub host: String,
    pub port: String,
    pub debug: bool,
    help: bool,
    version: bool,
}

impl Args {

    pub fn get() -> Args {
        let args: Vec<String> = env::args().collect();
        let mut flagged: Option<&str> = None;
        let mut out: Args = Args::default();
        args[1..].iter().enumerate().for_each(|(n, arg)| {
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
                    _ => {
                        println!("No valid option selected: {}", flag)
                    },
                }
            } else if flagged.is_none() {
                let cmd = args[n].as_str();
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
                    _ => println!("Invalid subcmd: {}", cmd),
                }
            }
        });
        out
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            debug: false,
            help: false,
            version: false,
            host: String::from("127.0.0.1"),
            port: String::from("8080"),
        }
    }
}
