use types::{Action, CommandType};

use commands::create::{CREATE, get_create_args};
use commands::join::{JOIN, get_join_args};
use commands::set::{SET, get_set_args};
use commands::list::{LIST, get_list_args};
use commands::quit::{QUIT};

use parser::sentence::{parse};

pub fn get_action(input: String) -> Result<Action, String> {
  match input.starts_with('/') {
    true => get_command_type(input)
      .and_then(|v| Ok(Action::Command(v))),
    _ => Ok(Action::Message(input)),
  }
}

fn get_command_type(c: String) -> Result<CommandType, String> {
  let (command, args) = parse(c);

  match &*command.to_lowercase() {
    CREATE => get_create_args(&args),
    JOIN => get_join_args(&args),
    SET => get_set_args(&args),
    LIST => get_list_args(&args),
    QUIT => Ok(CommandType::Quit),
    _ => Err("Command not implemented".into()),
  }
}
