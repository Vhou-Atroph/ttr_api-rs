#![deny(clippy::all)]
#![allow(non_snake_case)]
extern crate reqwest;
extern crate chrono;
use reqwest::Client;

/// Makes the default client for the API checker.

pub fn makeclient() -> Result<Client,reqwest::Error> {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"),);
    Client::builder().user_agent(APP_USER_AGENT).build()}


///Tools for Toontown Rewritten's Population API

pub mod Population {
    extern crate reqwest;
    extern crate serde;
    extern crate serde_json;
    extern crate chrono;
    use std::collections::HashMap;
    use reqwest::Client;
    use serde::Deserialize;
    use chrono::prelude::*;
    
    ///Struct for the Population API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/population.md>
    
    #[derive(Deserialize,Debug)]
    pub struct PopAPI {
        pub lastUpdated: i64,
        pub totalPopulation: u16,
        pub populationByDistrict: HashMap<String,u16>,}
    
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
    
    ///Gives some basic information about the current population of Toontown Rewritten using the Population API.
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

///Tools for Toontown Rewritten's Silly Meter API

pub mod SillyMeter {
    extern crate reqwest;
    extern crate serde;
    extern crate serde_json;
    use reqwest::Client;
    use serde::Deserialize;

    ///Struct for the Silly Meter API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/silly-meter.md>

    #[derive(Deserialize,Debug)]
    pub struct Meter {
        pub state: String,
        pub rewards: Vec<String>,
        pub rewardDescriptions: Vec<String>,
        pub rewardPoints: Vec<Option<u32>>,
        pub winner: Option<String>,
        pub hp: u32,
        pub nextUpdateTimestamp: i64,
        pub asOf: i64,
    }

    impl Meter {

        ///Grabs information from the Silly Meter API and converts it to the Meter struct.

        #[tokio::main]
        pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
            let resp =  client.get("https://www.toontownrewritten.com/api/sillymeter").send().await?
            .json::<Self>()
            .await?;
            Ok(resp)}
    }
}

///Tools for Toontown Rewritten's Invasions API

pub mod Invasions {
    extern crate reqwest;
    extern crate serde;
    extern crate serde_json;
    use std::collections::HashMap;
    use reqwest::Client;
    use serde::Deserialize;

    ///Struct for the Invasions API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/invasions.md>

    #[derive(Deserialize,Debug)]
    pub struct Invasion {
        pub error: Option<String>,
        pub invasions: HashMap<String,DistrictInv>,
        pub lastUpdated: i64,
    }

    ///Struct for each individual district's invasions

    #[derive(Deserialize,Debug)]
    pub struct DistrictInv {
        pub asOf: i64,
        pub r#type: String,
        pub progress: String,
    }

    impl Invasion {
        
        ///Grabs information from the Invasions API and converts it to the PopAPI struct.

        #[tokio::main]
        pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
            let resp =  client.get("https://www.toontownrewritten.com/api/invasions").send().await?
            .json::<Self>()
            .await?;
            Ok(resp)}
    }
}