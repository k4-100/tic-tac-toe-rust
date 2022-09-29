use rand::prelude::*;

mod utl;

fn main() {
  let human_player: utl::PlayerSign= utl::get_player_sign();
  println!("{:?}", human_player);

  let mut current_player: utl::PlayerSign = human_player.clone();
  let mut board : [&str; 9] = [
    "*", "*", "*",
    "*", "*", "*",
    "*", "*", "*",
  ];
  let mut rounds: u8 = board.len() as u8;
  

  while rounds > 0 {
    // input from player/computer
    utl::perform_round(&mut board, &human_player, &mut current_player, &mut rounds );

    // "render"
    println!("{}", utl::get_board(board) );
    if rounds == 0{
      println!("draw");
    }
  }
  
}
