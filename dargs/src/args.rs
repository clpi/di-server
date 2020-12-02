use std::convert::TryFrom;

pub enum ArgKind {
    SubCmd(&'static str),
    OneFlag(&'static str),
    TwoFlag(&'static str)
}

// impl TryFrom<&str> for ArgKind {
//     type Error = String;
//     fn try_from<T>(input: T) -> Result<Self, Self::Error> {
//         let input: &str = input.into();
//         for (i, c) in input.chars().enumerate() {
//             let res = match (i, c) {
//                 (0, '-') | (1, '-') => continue,
//                 (_, _) => break,
//                 // (0, _) => {return Ok(Self::SubCmd(input))},
//                 // (1, _) => {return Ok(Self::OneFlag(input.split_at(i).1))},
//                 // (2, _) => {return Ok(Self::TwoFlag(input.split_at(i).1))},
//                 // (n, _) => {return Err(input.split_at(n))},
//             };
//         }
//     }
// }
