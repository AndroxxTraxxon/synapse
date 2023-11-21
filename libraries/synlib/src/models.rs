use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StackStormPack {
    #[serde(rename = "ref")]
    pub pack_ref: String,
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub version: String,
    pub dependencies: Option<Vec<String>>, // Optional field
    pub author: String,
    pub email: String,
    pub contributors: Option<Vec<String>>, // Optional field
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub id: Option<Uuid>,
    pub name: String,
    #[serde(rename = "runner_type")]
    pub runner_type: String,
    pub description: String,
    pub enabled: bool,
    #[serde(rename = "entry_point")]
    pub entry_point: String,
    pub parameters: Option<HashMap<String, ParameterDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterDetail {
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
    pub required: bool,
    pub position: i32,
    pub secret: Option<bool>, // Optional, as it's not present in all parameters
    pub default: Option<String>, // Optional, as it's not present in all parameters
}
