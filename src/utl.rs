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


pub fn perform_round( board: [&str; 9], player_sign: PlayerSign   ){

}


pub fn get_board(board: [&str; 9] ) -> String{
  return format!("     
          ###########
          #{b0}###{b1}###{b2}#
          ###########
          #{b3}###{b4}###{b5}#
          ###########
          #{b6}###{b7}###{b8}#
          ###########", 
          b0 = board[0], b1 = board[1], b2 = board[2],
          b3 = board[3], b4 = board[4], b5 = board[5],
          b6 = board[6], b7 = board[7], b8 = board[8],
  );


}