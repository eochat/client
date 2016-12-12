use std::str::FromStr;
use types::{CommandType};

pub const LIST: &'static str = "/list";

#[derive(Serialize, Deserialize, Debug)]
pub enum ListType {
  Channels,
  Users,
}

impl FromStr for ListType {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "channels" => Ok(ListType::Channels),
      "users" => Ok(ListType::Users),
      _ => Err("First argument for /set can only be channels or users".into()),
    }
  }
}

pub fn get_list_args(args: &Vec<String>) -> Result<CommandType, String> {
  match args.len() {
    1 => match ListType::from_str(&args[0]) {
      Ok(a) => Ok(CommandType::List(a)),
      Err(e) => Err(e),
    },
    _ => Err("Create command requires 2 arguments".into()),
  }
}
