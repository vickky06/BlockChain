use rand::{seq::SliceRandom, Rng};

pub struct utils{
}

impl utils{

    pub fn generate_custom_random_string(length: usize) -> String {
        let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        let mut rng = rand::rng();
        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect()
    }
}