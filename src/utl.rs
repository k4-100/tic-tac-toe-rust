use std::io::{self, Write};

#[derive(Debug)]
pub enum PlayerSign {
  X,
  O,
  Null
}

pub fn get_player_sign() -> PlayerSign {
  let mut player_sign: PlayerSign = PlayerSign::Null;
  let mut buf: String;


  while matches!(player_sign, PlayerSign::Null){
    buf = String::new();
    print!("Pick X or O: ");
    io::stdout().flush().expect("error: failed to flush");
    io::stdin().read_line(&mut buf).expect("error: can't read console input");

    match buf.as_str().trim(){
      "X" | "x" => player_sign = PlayerSign::X,
      "O" | "o" => player_sign = PlayerSign::O,
      _=> {
        println!("Wrong input, try again");
      }
    }
  }
  
  return player_sign 
} 



pub fn get_board() -> String{

  return String::from("
          ###########
          #X###X###X#
          ###########
          #X###X###X#
          ###########
          #X###X###X#
          ###########")
}