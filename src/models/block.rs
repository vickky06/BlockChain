

#[derive(Debug,Clone)]
pub struct Block{
    id:u64,
    nonce:u64,
    data:String,
    hash:String,
    previous_hash:String,
    timestamp:i64
}
use chrono::Utc;
use sha256::digest;
impl Block{
    pub fn new(id:u64, previous_hash:String, data:String)->Self{
        let now = Utc::now();
        let now_time_stamp = now.timestamp();
        let (nonce, hash)  = Block :: mine_block(id, now_time_stamp, &previous_hash, &data);
        Self{
            id, 
            hash,
            timestamp:now.timestamp(),
            previous_hash,
            data,
            nonce
        }
    }
    pub fn get_hash(&self)->String{
        self.hash.clone()
    }
    pub fn get_previous_hash(&self)->String{
        self.previous_hash.clone()
    }

    pub fn get_id(&self)->u64{
        self.id
    }

    pub fn get_data(&self)->String{
        self.data.clone()
    }
    pub fn get_timestamp(&self)->i64{
        self.timestamp
    }
    pub fn get_nonce(&self)->u64{
        self.nonce
    }
    pub fn starting_block()->Self{
        Block{
            id :1 ,
            data: String::from("Genesis block"),
            previous_hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            nonce:11316,
            hash: String::from("000015783b764259d382017d91a36d206d0600e2cbb3567748f46a33fe9297cf"),
            timestamp:Utc::now().timestamp()
        }   
    }
    fn mine_block(id:u64, timestamp:i64, previous_hash:&str, data:&str)->(u64, String){
        println!("mining block with id:{}", id);
        let mut nonce = 01;
        loop {
            let block_string = format!(
                "{}{}{}{}{}", id, previous_hash.clone(), data.clone(),timestamp, nonce
            );
            let hash = digest(block_string);
            if hash.starts_with("0000"){
                println!("block mined with nonce {}", nonce);
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}