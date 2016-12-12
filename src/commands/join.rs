use types::{CommandType};

pub const JOIN: &'static str = "/join";

pub fn get_join_args(args: &Vec<String>) -> Result<CommandType, String> {
  match args.len() {
    1 => Ok(CommandType::Join(args[0].clone())),
    _ => Err("Join command requires an argument".into()),
  }
}
