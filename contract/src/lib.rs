// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};
//use near_sdk::serde_json::json;
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct MyContract {
    sum:u32,
}

#[near_bindgen]
impl MyContract {
    pub fn set_value(&mut self, value1: u32, value2:u32) {
        self.sum = value1+ value2;
        // env::log(format!("Value set to {}", value).as_u32());
    }

    pub fn get_value(&self) -> u32 {
        self.sum
    }
}