#![allow(non_snake_case)]
extern crate reqwest;
extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;

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

pub fn parse(api: &str) {
    match api {
        "population" => pop_api(makeclient().unwrap()).unwrap(),
        _ => panic!("Could not find {}!",api),
    };
}

#[tokio::main]
async fn pop_api(client:Client) -> Result<PopAPI,Box<dyn std::error::Error>> {
    let resp =  client.get("https://www.toontownrewritten.com/api/population").send().await?
    .json::<PopAPI>()
    .await?;
    println!("{:?}",resp);
    Ok(resp)
}