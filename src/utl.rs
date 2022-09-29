use std::{io::{self, Write}};
use std::num::{ParseIntError};
use rand::prelude::*;

#[derive(Debug,Clone, PartialEq)]
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


pub fn perform_round( board: &mut [&str; 9], human_player: &PlayerSign, current_player:  &mut PlayerSign, rounds: &mut u8 ){
  let mut passed: bool = false;
  let mut buf_num: usize;
  
  while !passed {
    let mut buf: String = String::new();
    if human_player == current_player{
      print!("pick a spot: ");
      io::stdout().flush().expect("failed to flush");
      io::stdin().read_line(&mut buf).expect("failed to readline");
      let buf_num_result: Result<usize, ParseIntError> = buf.trim().parse::<usize>();
      buf_num = match buf_num_result {
        Ok(v) => v,
        Err(_)=> {
          println!("Failed to parse to usize{buf}");
          0
        }
      };
    }
    else {
      println!("AI TIME");
      buf_num = rand::thread_rng().gen_range(1..9);
    }
    
    match buf_num{
      1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
        if board[buf_num-1] == "*"{
          
          if matches!(current_player, PlayerSign::X){

            board[buf_num-1] = "X";
            *current_player = PlayerSign::O;
          }else{
            board[buf_num-1] = "O";
            *current_player = PlayerSign::X;
          }
          *rounds -= 1;
          passed = true;
        }
        else{
          println!("index already taken");
        }
      }
      _ => println!("not an index")
    }
  }
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