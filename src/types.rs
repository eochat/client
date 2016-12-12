use commands::create::{Visibility};
use commands::set::{SetType};
use commands::list::{ListType};

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
  Command(CommandType),
  Message(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandType {
  Create(String, Visibility),
  Join(String),
  Set(SetType, String),
  List(ListType),
  Quit,
}
