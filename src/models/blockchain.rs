use sha256::digest;

use crate::models::block::Block;

#[derive(Debug, Clone)]
pub struct BlockChain {
    blocks: Vec<Block>,
}
impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::starting_block()],
        }
    }

    pub fn print_chain(&self) {
        for block in &self.blocks {
            println!("{:?}", block);
        }
    }

    fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.get_previous_hash() != latest_block.get_hash() {
            println!("block is not valid - previous Hash is invalid");
            return false;
        } else if !new_block.get_hash().starts_with("0000") {
            println!("block is not valid- critieria failed");
            return false;
        } else if !new_block.get_id() == latest_block.get_id() + 1 {
            println!("block is not valid - id is invalid");
            return false;
        } else if new_block.get_hash()
            != digest(format!(
                "{}{}{}{}{}",
                new_block.get_id(),
                new_block.get_previous_hash(),
                new_block.get_data(),
                new_block.get_timestamp(),
                new_block.get_nonce()
            ))
        {
            println!("block is not valid - hash is invalid");
            return false;
        } else {
            return true;
        }
    }
    pub fn try_add_block(&mut self, data:String) {
        let new_block = Block :: new(
            self.blocks.last().unwrap().get_id() + 1,
            self.blocks.last().unwrap().get_hash(),
            data,
        );
        let last_new_block = self.blocks.last().unwrap();
        match self.blocks.last() {
            None => {
                println!("blockchain is empty");
                return;
            }
            Some(last_block) => {
                if self.is_block_valid(&new_block, last_new_block) {
                    self.blocks.push(new_block);
                }
            }
        }
    }

    pub fn validate_chain(&self)->bool{
        for i in 1..self.blocks.len(){
            if !self.is_block_valid(&self.blocks[i], &self.blocks[i-1]){
                println!("blockchain is not valid");
                return false;
            }
        }
        println!("blockchain is valid");
        return true;
    }
}
