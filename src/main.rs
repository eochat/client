#![feature(proc_macro)]

extern crate serde;
extern crate serde_json;

#[macro_use] extern crate serde_derive;

mod commands;
mod parser;
mod utils;
mod types;

use utils::parser_helper::{get_action};
use types::{Action, CommandType};

fn main() {
  loop {
    let mut input = String::new();

    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let input = String::from(input.trim());

    match get_action(input) {
      Ok(a) => match a {
        Action::Command(t) => execute_command(t),
        Action::Message(m) => send_message(m)
      },
      Err(e) => println!("{}", e)
    }
  }
}

fn execute_command(c: CommandType) {
  println!("{}", serde_json::to_string_pretty(&c).unwrap())
}

fn send_message(msg: String) {
  println!("I will send a message")
}
