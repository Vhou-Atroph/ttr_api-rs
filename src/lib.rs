//!A library specialized for contacting Toontown Rewritten APIs. See an example usage of this crate at <https://github.com/Vhou-Atroph/ttr_pop_webserver>.
#![allow(non_snake_case)]
#![allow(dead_code)]
#![deny(clippy::all)]
extern crate reqwest;
use reqwest::Client;

///Makes the default client for the API checker.

pub fn makeclient() -> Client {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"),);
    Client::builder().user_agent(APP_USER_AGENT).build().unwrap()}

///Tools for Toontown Rewritten's Population API

pub mod Population {
    extern crate reqwest;
    extern crate serde;
    extern crate serde_json;
    use std::collections::HashMap;
    use reqwest::Client;
    use serde::Deserialize;
    
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
    
        pub fn highest_pop(&self) -> String {
            let dict = &self.populationByDistrict;
            let mut highest = String::new();
            let mut highest_count: u16 = 0;
            for (k,v) in dict.clone() {
                if v>highest_count {highest=k; highest_count=v;}
            } highest}
        
        ///Returns the current least popular district from the populationByDistrict HashMap.
    
        pub fn lowest_pop(&self) -> String {
            let dict = &self.populationByDistrict;
            let mut lowest = String::new();
            let mut lowest_count: u16 = 500;
            for (k,v) in dict.clone() {
                if v<lowest_count {lowest=k; lowest_count=v;}
            } lowest}
        }
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
        
        ///Detects if a particular cog is currently invading a district
        
        pub fn cog_invading(self,cog:&str) -> bool {
            for (_k,v) in self.invasions {
                if v.r#type == cog {return true}
            } false
        }
    }
}

///Tools for Toontown Rewritten's Field Office API

pub mod Offices {
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
        pub annexes: u8,
        pub expiring: Option<i64>,
    }

    impl Office {
        
        ///Grabs information from the Field Office API and converts it to the PopAPI struct.

        #[tokio::main]
        pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
            let resp =  client.get("https://www.toontownrewritten.com/api/fieldoffices").send().await?
            .json::<Self>()
            .await?;
            Ok(resp)}
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
}

///Tools for Toontown Rewritten's Doodle API

pub mod Doodles {
    extern crate reqwest;
    extern crate serde;
    extern crate serde_json;
    use std::collections::HashMap;
    use reqwest::Client;
    use serde::Deserialize;

    ///Struct for the Doodle API for Toontown Rewritten. See information regarding the API at <https://github.com/ToontownRewritten/api-doc/blob/master/doodles.md>
    
    #[derive(Deserialize,Debug)]
    pub struct Districts (HashMap<String,HashMap<String,Vec<Doodle>>>);

    ///Struct for information about a specific doodle.
    
    #[derive(Deserialize,Debug)]
    pub struct Doodle {
        pub dna: String,
        pub traits: Vec<String>,
        pub cost: u16,
    }

    impl Districts {

        ///Grabs information from the Doodle API and converts it to the Districts struct.

        #[tokio::main]
        pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
            let resp =  client.get("https://www.toontownrewritten.com/api/doodles").send().await?
            .json::<Self>()
            .await?;
            Ok(resp)}
        
        ///Get a specific doodle in a store today. If the district, playground, or doodle don't exist this function will return None.
        
        pub fn get_doodle(self,dist:&str,playground:&str,id:usize) -> Result<Doodle,Option<Doodle>> {
            if id>6 {return Err(None)}
            let dist_hash;
            let pg_vec;
            if self.0.contains_key(dist) {dist_hash=self.0.get(dist).unwrap()} else {return Err(None)}
            if dist_hash.contains_key(playground) {pg_vec=dist_hash.get(playground).unwrap()} else {return Err(None)}
            Ok(Doodle {
                dna: pg_vec[id].dna.clone(),
                traits: pg_vec[id].traits.clone(),
                cost: pg_vec[id].cost,
            })}
    }

    impl Doodle {

        ///Creates the link to a doodle rendition.
        /// The below function would return the render of a doodle from the pet shop in the Toontown Central of Blam Canyon.
        /// ```
        /// use ttr_api::Doodles;
        /// 
        /// fn doodle_render() -> String {
        ///     let doodle_api = Doodles::Districts::new(ttr_api::makeclient()).unwrap();
        ///     let doodle = doodle_api.get_doodle("Blam Canyon","Toontown Central",0).unwrap();
        ///     doodle.render(256,"png")
        /// }
        /// ```

        pub fn render(self,dim:u16,ext:&str) -> String {
            format!("rendition.toontownrewritten.com/render/{}/doodle/{}x{}.{}",self.dna,dim,dim,ext)
        }

    }
}