//! Tools for Toontown Rewritten's Invasions API

extern crate reqwest;
extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;

/// Struct for the Invasions API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/invasions.md>

#[derive(Deserialize,Debug)]
pub struct Invasion {
    pub error: Option<String>,
    pub invasions: HashMap<String,DistrictInv>,
    pub lastUpdated: i64,
}

/// Struct for each individual district's invasions

#[derive(Deserialize,Debug)]
pub struct DistrictInv {
    pub asOf: i64,
    pub r#type: String,
    pub progress: String,
}

impl Invasion {
    
    /// Grabs information from the Invasions API and converts it to the Invasion struct.

    #[tokio::main]
    pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/invasions").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }
    
    /// Detects if a particular cog is currently invading a district and returns a tuple containing a boolean and an Option value.
    /// ```
    /// use ttr_api::invasion::Invasion
    /// 
    /// fn minglers_invading() -> Option<String> {
    ///     let invasions = Invasion::new().unwrap();
    ///     let minglers_district = invasions.cog_invading("The Mingler");
    ///     
    /// }
    /// ```
    
    pub fn cog_invading(&self,cog:&str) -> (bool,Option<String>,Option<&DistrictInv>) {
        for (k,v) in &self.invasions {
            if v.r#type == cog.to_string() {return (true,Some(k.clone()),Some(v))}
        } (false,None,None)
    }
}