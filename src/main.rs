use crate::utl::get_board;

mod utl;

fn main() {
  let player_sign: utl::PlayerSign= utl::get_player_sign();
  println!("{:?}", player_sign);
  let current_player: utl::PlayerSign = player_sign;
  while true{

    // "render"
    println!("{}", utl::get_board() );
  }
  
}
