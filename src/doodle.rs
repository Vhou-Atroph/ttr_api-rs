//! Tools for Toontown Rewritten's Doodle API

extern crate reqwest;
extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;

/// Struct for the Doodle API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/doodles.md>
#[derive(Deserialize,Debug)]
pub struct Districts (HashMap<String,HashMap<String,Vec<Doodle>>>);

/// Struct for information about a specific doodle.
#[derive(Deserialize,Debug)]
pub struct Doodle {
    pub dna: String,
    pub traits: Vec<String>,
    pub cost: u16,
}

impl Districts {

    /// Grabs information from the Doodle API and converts it to the Districts struct.
    /// ```
    /// use ttr_api::doodle;
    /// 
    /// let doodle_api = doodle::Districts::new(ttr_api::makeclient()).unwrap();
    /// ```
    #[tokio::main]
    pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/doodles").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }
    
    /// Get a specific doodle in a store today. If the district, playground, or doodle don't exist this function will return None.
    pub fn get_doodle(&self,dist:&str,playground:&str,id:usize) -> Option<Doodle> {
        if id>6 {return None}
        let dist_hash;
        let pg_vec;
        if self.0.contains_key(dist) {dist_hash=self.0.get(dist).unwrap()} else {return None}
        if dist_hash.contains_key(playground) {pg_vec=dist_hash.get(playground).unwrap()} else {return None}
        Some(Doodle {
            dna: pg_vec[id].dna.clone(),
            traits: pg_vec[id].traits.clone(),
            cost: pg_vec[id].cost,
        })
    }
}