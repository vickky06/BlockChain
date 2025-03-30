mod models;
mod utils;

use crate::models::{ blockchain::BlockChain};
fn main() {
    let mut block_chain = BlockChain::new();
   block_chain.print_chain();
   for _ in 0..10 {
    block_chain.try_add_block(utils::generator::utils::generate_custom_random_string(10));
}
   block_chain.try_add_block(
    String::from("last block")
   );
   block_chain.validate_chain();
   block_chain.print_chain();

   
}
