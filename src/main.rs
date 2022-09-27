use crate::utl::get_board;

mod utl;

fn main() {
  let player_sign: utl::PlayerSign= utl::get_player_sign();
  println!("{:?}", player_sign);

  let mut current_player: utl::PlayerSign = player_sign;
  let mut board : [&str; 9] = [
    "*", "*", "*",
    "*", "*", "*",
    "*", "*", "*",
  ];

  while true{

    utl::perform_round(&mut board, &mut current_player);

    // "render"
    println!("{}", utl::get_board(board) );
  }
  
}
