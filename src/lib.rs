extern crate reqwest;
extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use reqwest::{Client,get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug)]
pub struct PopAPI {
    lastUpdated: u32,
    totalPopulation: u16,
    populationByDistrict: HashMap<String,u16>,
}

pub fn makeclient() -> Result<Client,reqwest::Error> {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"),);
    let client = Client::builder().user_agent(APP_USER_AGENT).build();
    client}

#[tokio::main]
pub async fn get_json() -> Result<PopAPI,Box<dyn std::error::Error>> {
    let resp =  get("https://www.toontownrewritten.com/api/population").await?
    .json::<PopAPI>()
    .await?;
    println!("{:?}",resp);
    Ok(resp)
}