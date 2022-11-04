//! A library specialized for contacting Toontown Rewritten APIs. See an example usage of this crate at <https://github.com/Vhou-Atroph/ttr_pop_webserver>.
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![deny(clippy::all)]
extern crate reqwest;
use reqwest::Client;

/// Makes the default client for the API checker. Clients are required to access the API, and this one is provided in case one does not wish to make their own.
pub fn makeclient() -> Client {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"),);
    Client::builder().user_agent(APP_USER_AGENT).build().unwrap()
}

pub mod population;
pub mod sillymeter;
pub mod invasion;
pub mod office;
pub mod doodle;
pub mod news;
pub mod releasenotes;
pub mod rendition;