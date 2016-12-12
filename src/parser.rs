use std::str::FromStr;

pub enum Action {
  Command(CommandType),
  Message(String)
}

enum Visibility {
  Public,
  Private
}

enum SetType {
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

pub enum CommandType {
  Create(String, Visibility),
  Join(String),
  Set(SetType, String),
  Leave,
  Quit,
  Unknown
}

const CREATE: &'static str = "/create";
const JOIN: &'static str = "/join";
const SET: &'static str = "/set";
const LEAVE: &'static str = "/leave";
const QUIT: &'static str = "/quit";

pub fn get_action(input: String) -> Result<Action, String> {
  match input.starts_with('/') {
    true => get_command_type(input)
      .and_then(|v| Ok(Action::Command(v))),
    _ => Ok(Action::Message(input))
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
    LEAVE => Ok(CommandType::Leave),
    QUIT => Ok(CommandType::Quit),
    _ => Err("Command not implemented".into())
  }
}

fn get_create_args(args: &Vec<String>) -> Result<CommandType, String> {
  match args.len() {
    2 => match Visibility::from_str(&args[1]) {
      Ok(a) => Ok(CommandType::Create(args[0].clone(), a)),
      Err(e) => Err(e),
    },
    _ => Err("Create command requires 2 arguments".into())
  }
}

fn get_join_args(args: &Vec<String>) -> Result<CommandType, String> {
  match args.len() {
    1 => Ok(CommandType::Join(args[0].clone())),
    _ => Err("Join command requires an argument".into()),
  }
}

fn get_set_args(args: &Vec<String>) -> Result<CommandType, String> {
  match args.len() {
    2 => match SetType::from_str(&args[0]) {
      Ok(a) => Ok(CommandType::Set(a, args[1].clone())),
      Err(e) => Err(e),
    },
    _ => Err("Set command requires 2 arguments".into())
  }
}


//
//enum CommandKind {
//  Create(String, Visibility),
//  Join(String),
//  Set(String, String),
//  Leave,
//  Quit
//}
