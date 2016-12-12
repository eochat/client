use std::str::FromStr;
use types::{CommandType};

pub const CREATE: &'static str = "/create";

#[derive(Serialize, Deserialize, Debug)]
pub enum Visibility {
  Public,
  Private,
}

impl FromStr for Visibility {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "public" => Ok(Visibility::Public),
      "private" => Ok(Visibility::Private),
      _ => Err("First argument for /create can only be public or private".into()),
    }
  }
}

pub fn get_create_args(args: &Vec<String>) -> Result<CommandType, String> {
  match args.len() {
    2 => match Visibility::from_str(&args[1]) {
      Ok(a) => Ok(CommandType::Create(args[0].clone(), a)),
      Err(e) => Err(e),
    },
    _ => Err("Create command requires 2 arguments".into()),
  }
}
