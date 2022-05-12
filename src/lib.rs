extern crate reqwest;
extern crate serde;
use std::collections::HashMap;
use reqwest::{Client,get};
use serde::{Deserialize, Serialize};
pub fn makeclient() -> Result<Client,reqwest::Error> {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"),);
    let client = Client::builder().user_agent(APP_USER_AGENT).build();
    client}