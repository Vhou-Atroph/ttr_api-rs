//! Tools for Toontown Rewritten's Rendition API

use crate::doodle;

pub trait Render {
    fn render(&self,dim:usize,format:Format) -> String;
}

/// File formats accepted by the Rendition API.
pub enum Format {
    Png,
    Webp,
}

impl Render for doodle::Doodle {

    /// Creates the link to a doodle rendition.
    /// The below function would return the render of a doodle from the pet shop in the Toontown Central of Blam Canyon.
    /// ```
    /// use ttr_api::{doodle,rendition::*};
    /// 
    /// fn doodle_render() -> String {
    ///     let doodle_api = doodle::Districts::new(ttr_api::makeclient()).unwrap();
    ///     let doodle = doodle_api.get_doodle("Blam Canyon","Toontown Central",0).unwrap();
    ///     doodle.render(256,Format::Png)
    /// }
    /// ```

    fn render(&self,dim:usize,format:Format) -> String {
        match format {
            Format::Png => format!("https://rendition.toontownrewritten.com/render/{}/doodle/{}x{}.png",self.dna,dim,dim),
            Format::Webp => format!("https://rendition.toontownrewritten.com/render/{}/doodle/{}x{}.webp",self.dna,dim,dim)
        }
    }
}