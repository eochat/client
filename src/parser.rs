use types::{Action, CommandType};

use commands::create::{CREATE, get_create_args};
use commands::join::{JOIN, get_join_args};
use commands::set::{SET, get_set_args};
use commands::list::{LIST, get_list_args};
use commands::quit::{QUIT};

pub fn get_action(input: String) -> Result<Action, String> {
  match input.starts_with('/') {
    true => get_command_type(input)
      .and_then(|v| Ok(Action::Command(v))),
    _ => Ok(Action::Message(input)),
  }
}

fn parse_sentence(sentence: String) -> (String, Vec<String>) {
  let mut words = sentence.split_whitespace();

  let command: String = words
    .nth(0)
    .map(ToString::to_string)
    .map(|s| s.to_lowercase())
    .unwrap_or(String::from("unknown"));

  let params: Vec<String> = words
    .skip(0)
    .map(|s| s.to_lowercase())
    .collect();

  (command, params)
}

fn get_command_type(c: String) -> Result<CommandType, String> {
  let (command, args) = parse_sentence(c);

  match &*command.to_lowercase() {
    CREATE => get_create_args(&args),
    JOIN => get_join_args(&args),
    SET => get_set_args(&args),
    LIST => get_list_args(&args),
    QUIT => Ok(CommandType::Quit),
    _ => Err("Command not implemented".into()),
  }
}
