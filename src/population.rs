//! Tools for Toontown Rewritten's Population API

extern crate reqwest;
extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;

/// Struct for the Population API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/population.md>

#[derive(Deserialize,Debug)]
pub struct PopAPI {
    pub lastUpdated: i64,
    pub totalPopulation: u16,
    pub populationByDistrict: HashMap<String,u16>,
    pub statusByDistrict: HashMap<String,String>,
}

impl PopAPI {

    /// Grabs information from the Population API and converts it to the PopAPI struct.
    #[tokio::main]
    pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/population").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }


    /// Returns the current most popular district from the populationByDistrict HashMap.
    pub fn highest_pop(&self) -> String {
        let dict = &self.populationByDistrict;
        let mut highest = String::new();
        let mut highest_count: u16 = 0;
        for (k,v) in dict.clone() {
            if v>highest_count {highest=k; highest_count=v;}
        } highest
    }
    
    /// Returns the current least popular district from the populationByDistrict HashMap.
    pub fn lowest_pop(&self) -> String {
        let dict = &self.populationByDistrict;
        let mut lowest = String::new();
        let mut lowest_count: u16 = 500;
        for (k,v) in dict.clone() {
            if v<lowest_count {lowest=k; lowest_count=v;}
        } lowest
    }
    
    /// Gets the population of a specific district. If the district does not exist, returns None.
    /// ```
    /// use ttr_api::population;
    /// 
    /// fn blam_pop() {
    ///     let pop_api = population::PopAPI::new(ttr_api::makeclient()).unwrap();
    ///     let blam_pop = pop_api.dist_pop("Blam Canyon").unwrap();
    ///     println!("Blam Canyon currently has a population of {} toons.",blam_pop);
    /// }
    /// ```
    pub fn dist_pop(&self,dist:&str) -> Option<u16> {
        let pop;
        if self.populationByDistrict.contains_key(dist) {pop = self.populationByDistrict.get(dist).unwrap();}
        else {return None} Some(*pop)}
}