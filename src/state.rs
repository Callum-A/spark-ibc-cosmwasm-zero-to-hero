use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr, // juno1xyz
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub question: String,
    pub yes_votes: u64,
    pub no_votes: u64,
}

pub const CONFIG: Item<Config> = Item::new("config"); // This is stored on chain!

// String -> Poll
// "Do you love Spark IBC?" -> Poll {
//                              question: "Do you love Spark IBC?",
//                              yes_votes: 100,
//                              no_votes: 50
//                             }
pub const POLLS: Map<String, Poll> = Map::new("polls"); // Stores poll variables, with a string index
