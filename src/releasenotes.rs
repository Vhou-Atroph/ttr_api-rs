//!Tools for Toontown Rewritten's Release Notes API

extern crate reqwest;
extern crate serde;
extern crate serde_json;
use reqwest::Client;
use serde::Deserialize;

///Struct for and individual entry in the release notes API for Toontown Rewritten. It does not have any formal documentation. You can find it at <https://www.toontownrewritten.com/api/releasenotes> or <https://www.toontownrewritten.com/api/releasenotes/[id]>.

#[derive(Deserialize,Debug,Clone)]
pub struct Release {
    pub noteId: u16,
    pub slug: String,
    pub date: String,
    pub body: Option<String>
}

///Struct for the release notes API for Toontown Rewritten. It does not have any formal documentation. You can find it at <https://www.toontownrewritten.com/api/releasenotes>.

#[derive(Deserialize,Debug)]
pub struct NotesList(Vec<Release>);

impl Release {

    ///Get specific release notes by noteID.
    
    #[tokio::main]
    pub async fn new(client:Client,id:u16) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get(format!("https://www.toontownrewritten.com/api/releasenotes/{}",id)).send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }
}

impl NotesList {

    ///Grabs a complete list of release notes from the API.
    
    #[tokio::main]
    pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/releasenotes").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)}

    ///Grabs a specific note index from NotesList.
    
    pub fn get_index(&self,index:usize) -> Release {
        self.0[index].clone()
    }
}