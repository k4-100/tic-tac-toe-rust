use std::io::{self, Write};

fn main() {
  let mut input: String = String::new();
  println!("WELCOME TO APP!");
  print!("Name: ");
  io::stdout().flush().expect("error: failed to flush after print!");
  let stdin = io::stdin().read_line(&mut input).expect("error: can't read console input");
  println!("\n\nWelcome {} !", input.trim());
}
