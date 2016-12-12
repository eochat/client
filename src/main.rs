mod parser;

use parser::{Action,CommandType};

fn main() {
  let mut input = String::new();

  std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let input = String::from(input.trim());

  match parser::get_action(input) {
    Ok(a) => match a {
      Action::Command(t) => execute_command(t),
      Action::Message(m) => send_message(m)
    },
    Err(e) => println!("{}", e)
  }
}

fn execute_command(c: CommandType) {
  println!("I will execute a command")
}

fn send_message(msg: String) {
  println!("I will send a message")
}
