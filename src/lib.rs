#![allow(non_snake_case)]
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;
use chrono::{prelude::*,offset};

pub fn makeclient() -> Result<Client,reqwest::Error> {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"),);
    Client::builder().user_agent(APP_USER_AGENT).build()}
pub fn parse(api: &str) {
    match api {
        "population" => println!("{}",Population::pop_info(&Population::pop_api(makeclient().unwrap()).unwrap())),
        _ => panic!("Could not find {}!",api),};}

mod Population {    
    use super::*;
    #[derive(Deserialize,Debug)]
    pub struct PopAPI {
        lastUpdated: i64,
        totalPopulation: u16,
        populationByDistrict: HashMap<String,u16>,}

    #[tokio::main]
    pub async fn pop_api(client:Client) -> Result<PopAPI,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/population").send().await?
        .json::<PopAPI>()
        .await?;
        Ok(resp)}
    pub fn pop_info(json:&PopAPI) -> String {
        let updated = NaiveDateTime::from_timestamp(json.lastUpdated,0);
        let updated_time: DateTime<offset::Utc> = DateTime::from_utc(updated,offset::Utc);
        let most_popular = highest_pop(&json.populationByDistrict);
        let least_popular = lowest_pop(&json.populationByDistrict);
        let info = format!("Last updated: {}\n\
        Total Population: {}\n\
        Current most popular district: {} with {} toons.\n\
        Current least popular district: {} with {} toons.\n",
        updated_time,json.totalPopulation,most_popular,json.populationByDistrict.get(&most_popular).unwrap(),least_popular,json.populationByDistrict.get(&least_popular).unwrap());
        info.to_string()}
    fn highest_pop(dict:&HashMap<String,u16>) -> String {
        let mut highest = String::new();
        let mut highest_count: u16 = 0;
        for (k,v) in dict.clone() {
            if v>highest_count {highest=k; highest_count=v;}
        } highest}
    fn lowest_pop(dict:&HashMap<String,u16>) -> String {
        let mut lowest = String::new();
        let mut lowest_count: u16 = 500;
        for (k,v) in dict.clone() {
            if v<lowest_count {lowest=k; lowest_count=v;}
        } lowest}
}