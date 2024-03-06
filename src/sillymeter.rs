//! Tools for Toontown Rewritten's Silly Meter API

extern crate reqwest;
extern crate serde;
extern crate serde_json;
use reqwest::Client;
use serde::Deserialize;

/// Struct for the Silly Meter API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/silly-meter.md>

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

    /// Grabs information from the Silly Meter API and converts it to the Meter struct.
    #[tokio::main]
    pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/sillymeter").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }
}