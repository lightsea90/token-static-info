/*
Functions:
    * 
 */


// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, near_bindgen, assert_one_yocto};
use near_sdk::collections::UnorderedMap;
use near_sdk::AccountId;
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde_json::{json, Value};

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Campaign {
    sale_contract: AccountId,
    reference: Option<String>,
    reference_hash: Option<Base64VecU8>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId,
    campaigns: UnorderedMap<u64, Campaign>,
    counter: u64,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner_id: String::from("__default_value__"),
            campaigns: UnorderedMap::new(b"__default__".to_vec()),
            counter: 0,
        }
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        return Self {
            owner_id: owner_id,
            campaigns: UnorderedMap::new(b"c".to_vec()),
            counter: 0
        };
    }

    fn assert_owner_id(&self) {
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "Caller is not owner",
        );
    }

    #[payable]
    pub fn add_campaign(&mut self, campaign: Campaign) {        
        assert_one_yocto();
        self.assert_owner_id();
        self.counter += 1;
        self.campaigns.insert(&self.counter, &campaign);
    }

    #[payable]
    pub fn remove_campaign(&mut self, id: u64) {
        assert_one_yocto();
        self.assert_owner_id();
        if let Some(_) = self.campaigns.get(&id) {
            self.campaigns.remove(&id);
        } else {
            env::panic(format!("Campaign {} not existed", id).as_bytes())
        }
        
    }

    pub fn get_campaign(&self, id: u64) -> Campaign {
        if let Some(camp) = self.campaigns.get(&id) {
            camp
        } else {
            env::panic(format!("Campaign {} not existed", id).as_bytes())
        }
    }

    pub fn get_campaign_list(&self, from_index: u64, limit: u64) -> Value {
        let keys = self.campaigns.keys_as_vector();
        let values = self.campaigns.values_as_vector();
        let mut result = json!([]);
        for index in from_index..std::cmp::min(from_index + limit, self.campaigns.len()) {
            let id = keys.get(index).unwrap();
            let campaign = values.get(index).unwrap();
            result.as_array_mut().unwrap().push(json!([id, campaign]));
        }
        return json!({
            "total": keys.len(),
            "result": result,
        });
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

}

