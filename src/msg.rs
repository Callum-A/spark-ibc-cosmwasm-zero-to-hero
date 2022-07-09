use crate::state::Poll;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// How do we communicate with our contract

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin_address: String, // Why is this String not Addr? So we can validate it!
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePoll {
        // ExecuteMsg::CreatePoll { question: "Do you love Spark IBC?" }
        question: String,
    },
    Vote {
        question: String, // what question are we responding too?
        choice: String,   // what is our answer? "yes" or "no"
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPoll { question: String },
    GetConfig {},
}

// This is what we return for our GetPoll route
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetPollResponse {
    pub poll: Option<Poll>, // Option means it can either be null (None) or a Poll
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
