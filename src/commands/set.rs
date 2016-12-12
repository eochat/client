use std::str::FromStr;
use types::{CommandType};

pub const SET: &'static str = "/set";

#[derive(Serialize, Deserialize, Debug)]
pub enum SetType {
  Nickname,
}

impl FromStr for SetType {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "nickname" => Ok(SetType::Nickname),
      _ => Err("First argument for /set can only be nickname".into()),
    }
  }
}

pub fn get_set_args(args: &Vec<String>) -> Result<CommandType, String> {
  match args.len() {
    2 => match SetType::from_str(&args[0]) {
      Ok(a) => Ok(CommandType::Set(a, args[1].clone())),
      Err(e) => Err(e),
    },
    _ => Err("Set command requires 2 arguments".into()),
  }
}
