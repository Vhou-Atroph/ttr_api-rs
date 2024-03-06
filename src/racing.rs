//! Tools for Toontown Rewritten's Racing Leaderboards API

extern crate reqwest;
extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;

use crate::dna::DNA;

/// Struct for the highest division of the Racing Leaderboards API. You can find the API at <https://toontownrewritten.com/api/racing>.
#[derive(Deserialize, Debug)]
pub struct RacingLeaderboards {
    //pub today: ,
    //pub current_week: ,
    //pub previous_week: ,
    pub all_time: HashMap<String, HashMap<String, Vec<LeaderboardMember>>>
}

impl RacingLeaderboards {

    /// Grabs information from the Racing Leaderboards API and converts it to the RacingLeaderboards struct.
    #[tokio::main]
    pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/racing").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }
}

/// Struct for an individual entry on the Racing Leaderboard.
#[derive(Deserialize, Debug)]
pub struct LeaderboardMember(String, DNA, f64);