use std::{
    convert::TryFrom,
    env,
};

pub enum Arg {
    SubCmd(&'static str),
    OneFlag(&'static str),
    TwoFlag(&'static str)
}

pub struct Args {
    args: Vec<String>,
}

impl Args {
    pub fn get() -> Self {
        let mut args: Vec<String> = Vec::new();
        let mut env_args = env::args();
        while let Some(arg) = env_args.next() {
            args.push(arg);
        }
        Self { args: Vec::new() }
    }
}

impl TryFrom<&'static str> for Arg {
    type Error = String;
    fn try_from(input: &'static str) -> Result<Self, Self::Error> {
        let ind: (&str, &str) = input.split_at(1);
        let res = match ind.0.split_at(1) {
            ("-", "-") => Self::TwoFlag(&ind.1),
            ("-", _) => Self::OneFlag(&input[1..]),
            (_, _) => Self::SubCmd(input),
        };
        Ok(res)
    }
}
