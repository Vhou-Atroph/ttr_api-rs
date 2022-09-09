//!Tools for Toontown Rewritten's Field Office API

extern crate reqwest;
extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;

///Struct for the Field Office API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/field-offices.md>

#[derive(Deserialize,Debug)]
pub struct Office {
    pub lastUpdated: i64,
    pub fieldOffices: HashMap<u16,HQ>
}

///Struct for each individual street's field office

#[derive(Deserialize,Debug)]
pub struct HQ {
    pub department: char,
    pub difficulty: u8,
    pub annexes: u16,
    pub expiring: Option<i64>,
}

impl Office {
    
    ///Grabs information from the Field Office API and converts it to the Offices struct.

    #[tokio::main]
    pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/fieldoffices").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }
    
    ///Get a field office on a specific street. If it doesn't exist, returns None. Also returns the street ID.
    /// ```
    /// use ttr_api::office;
    /// 
    /// fn walrus_way_office() {
    ///     let office_api = office::Office::new(ttr_api::makeclient()).unwrap();
    ///     let walfice = office_api.get_office(3100);
    ///     match walfice.0 {
    ///         Some(HQ) => println!("There's a Field Office on Walrus Way!"),
    ///         None => println!("There's no Field Office on Walrus Way...")
    ///     }
    /// }
    /// ```
    
    pub fn get_office(&self,street:u16) -> (Option<HQ>,u16) {
        let hq_hash;
        if self.fieldOffices.contains_key(&street) 
            {hq_hash=self.fieldOffices.get(&street).unwrap();
            (Some(HQ {
                department: hq_hash.department,
                difficulty: hq_hash.difficulty,
                annexes: hq_hash.annexes,
                expiring: hq_hash.expiring,}),street)} 
        else {(None,street)}
    }
}

///Converts the locale id of a Field Office into a street name if it exists.

pub fn locale(id:u16) -> Option<String> {
    match id {
        3100 => Some(String::from("Walrus Way")),
        3200 => Some(String::from("Sleet Street")),
        3300 => Some(String::from("Polar Place")),
        4100 => Some(String::from("Alto Avenue")),
        4200 => Some(String::from("Baritone Boulevard")),
        4300 => Some(String::from("Tenor Terrace")),
        5100 => Some(String::from("Elm Street")),
        5200 => Some(String::from("Maple Street")),
        5300 => Some(String::from("Oak Street")),
        9100 => Some(String::from("Lullaby Lane")),
        9200 => Some(String::from("Pajama Place")),
        _ => None,
    }
}