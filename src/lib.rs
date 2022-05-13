#![deny(clippy::all)]
#![allow(non_snake_case)]
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;
use chrono::prelude::*;

/// Makes the default client for the API checker.

pub fn makeclient() -> Result<Client,reqwest::Error> {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"),);
    Client::builder().user_agent(APP_USER_AGENT).build()}

pub mod Population {    
    use super::*;
    
    ///Struct for the Population API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/population.md>
    
    #[derive(Deserialize,Debug)]
    pub struct PopAPI {
        lastUpdated: i64,
        totalPopulation: u16,
        populationByDistrict: HashMap<String,u16>,}
    
    impl PopAPI {

        ///Grabs information from the Population API and converts it to the PopAPI struct.

        #[tokio::main]
        pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
            let resp =  client.get("https://www.toontownrewritten.com/api/population").send().await?
            .json::<Self>()
            .await?;
            Ok(resp)}

    
        ///Returns the current most popular district from the populationByDistrict HashMap.
    
        fn highest_pop(&self) -> String {
            let dict = &self.populationByDistrict;
            let mut highest = String::new();
            let mut highest_count: u16 = 0;
            for (k,v) in dict.clone() {
                if v>highest_count {highest=k; highest_count=v;}
            } highest}
        
        ///Returns the current least popular district from the populationByDistrict HashMap.
    
        fn lowest_pop(&self) -> String {
            let dict = &self.populationByDistrict;
            let mut lowest = String::new();
            let mut lowest_count: u16 = 500;
            for (k,v) in dict.clone() {
                if v<lowest_count {lowest=k; lowest_count=v;}
            } lowest}
        }
    
    ///Gives some basic information about the current population of Toontown Rewritten using the Population API. Example usage with println:
    /// 
    /// ```
    /// use ttr_api_rs::Population;
    /// let pop = Population::PopAPI::new(ttr_api_rs::makeclient().unwrap()).unwrap();
    /// println!("{}",Population::pop_info(&pop));
    /// ```
    
    pub fn pop_info(json:&PopAPI) -> String {
        let updated = NaiveDateTime::from_timestamp(json.lastUpdated,0);
        let updated_time: DateTime<Utc> = DateTime::from_utc(updated,Utc);
        let most_popular = json.highest_pop();
        let least_popular = json.lowest_pop();
        let info = format!("Last updated: {}\n\
        Total Population: {}\n\
        Current most popular district: {} with {} toons.\n\
        Current least popular district: {} with {} toons.\n",
        updated_time,json.totalPopulation,most_popular,json.populationByDistrict.get(&most_popular).unwrap(),least_popular,json.populationByDistrict.get(&least_popular).unwrap());
        info.to_string()}
}