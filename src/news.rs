//! Tools for Toontown Rewritten's News API

extern crate reqwest;
extern crate serde;
extern crate serde_json;
use reqwest::Client;
use serde::Deserialize;

/// Struct for the News article API for Toontown Rewritten. It does not have any formal documentation. You can find it at <https://www.toontownrewritten.com/api/news> or <https://www.toontownrewritten.com/api/news/[id]>.
#[derive(Deserialize,Debug,Clone)]
pub struct News {
    pub title: String,
    pub postId: u16,
    pub author: String,
    pub body: Option<String>,
    pub date: String,
    pub image: String,
}

/// Struct for the News list API for Toontown Rewritten. It does not have any formal documentation. You can find it at <https://www.toontownrewritten.com/api/news/list>.
#[derive(Deserialize,Debug)]
pub struct NewsList(Vec<News>);

impl News {

    /// Grabs the latest news article from the API.
    #[tokio::main]
    pub async fn new_latest(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/news").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }

    /// Grabs a news article with specific ID from the API.
    #[tokio::main]
    pub async fn new_id(client:Client,id:u16) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get(format!("https://www.toontownrewritten.com/api/news/{}",id)).send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }
    
    /// Grabs link for a news article from the API.
    pub fn get_link(&self) -> String {
        format!("https://www.toontownrewritten.com/news/item/{}",self.postId)
    }
}

impl NewsList {

    /// Grabs a complete list of articles from the API.
    #[tokio::main]
    pub async fn new(client:Client) -> Result<Self,Box<dyn std::error::Error>> {
        let resp =  client.get("https://www.toontownrewritten.com/api/news/list").send().await?
        .json::<Self>()
        .await?;
        Ok(resp)
    }

    /// Grabs a specific article index from NewsList.
    pub fn get_index(&self,index:usize) -> News {
        self.0[index].clone()
    }
}