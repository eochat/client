enum Action {
  Command(CommandType),
  Message(String)
}

enum CommandType {
  Create,
  Join,
  Set,
  Leave,
  Quit,
  Unknown
}

fn main() {
  let mut input = String::new();

  std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let input = String::from(input.trim());

  match get_action(input) {
    Action::Command(t) => execute_command(t),
    Action::Message(m) => send_message(m)
  }
}

fn get_action(input: String) -> Action {
  match input.starts_with('/') {
    true => Action::Command(get_command_type(input)),
    _ => Action::Message(input)
  }
}

fn execute_command(c: CommandType) {
  println!("I will execute a command")
}

fn send_message(msg: String) {
  println!("I will send a message")
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

const CREATE: &'static str = "/create";
const JOIN: &'static str = "/join";
const SET: &'static str = "/set";
const LEAVE: &'static str = "/leave";
const QUIT: &'static str = "/quit";

fn get_command_type(c: String) -> CommandType {
  let (command, args) = parse_sentence(c);

  match &*command.to_lowercase() {
    CREATE => CommandType::Create,
    JOIN => CommandType::Join,
    SET => CommandType::Set,
    LEAVE => CommandType::Leave,
    QUIT => CommandType::Quit,
    _ => CommandType::Unknown,
  }
}

//enum Visibility {
//  Public,
//  Private
//}
//
//enum CommandKind {
//  Create(String, Visibility),
//  Join(String),
//  Set(String, String),
//  Leave,
//  Quit
//}
