use ttr_api::*;

#[test]
#[ignore]
fn is_cog_invading() {
    let inv = invasion::Invasion::new(ttr_api::makeclient()).unwrap();
    let test = inv.cog_invading("Cold Caller"); //Change this to a cog that is currently invading.
    assert_eq!(test.0,true)
}
#[test]
fn is_cog_not_invading() {
    let inv = invasion::Invasion::new(ttr_api::makeclient()).unwrap();
    let test = inv.cog_invading("Director of Ambush Marketing");
    assert_eq!(test.0,false)
}
#[test]
fn correct_fo_zone() {
    let walway = 3100;
    assert_eq!(office::locale(walway),Some(String::from("Walrus Way")));
}
#[test]
fn invalid_zone() {
    let zone = 2100;
    assert_eq!(office::locale(zone),None);
}
#[test]
fn doodle_render() {
    let doodle = doodle::Doodle {dna:String::from("64ff02ff01030e010001"),traits:vec!(String::from("godawful")),cost:1};
    let dod = doodle::Doodle::render(&doodle,256,"png");
    assert_eq!(dod,String::from("rendition.toontownrewritten.com/render/64ff02ff01030e010001/doodle/256x256.png"));
}
#[test]
#[ignore]
fn real_doodle_render() {
    let doodle_api = doodle::Districts::new(ttr_api::makeclient()).unwrap();
    let doodle = doodle_api.get_doodle("Blam Canyon","Toontown Central",0).unwrap();
    assert_eq!(doodle.render(256,"png"),String::from("rendition.toontownrewritten.com/render/64000302010102050001/doodle/256x256.png")) //Needs to be a real doodle in Blam Canyon, Toontown Central on the day it's tested.
}
#[test]
#[ignore]
fn latest_article() {
    let news_api = news::News::new_latest(ttr_api::makeclient()).unwrap();
    let post_id = news_api.postId;
    assert_eq!(post_id,773); //Needs to match latest post ID
}
#[test]
fn earliest_author() {
    let news_api = news::News::new_id(ttr_api::makeclient(),2).unwrap(); //The earliest news article ID is 2- I'm guessing id 1 is reserved for the original TTR announcement on MMOCF?
    assert_eq!(news_api.author,String::from("Sir Max"));
}
#[test]
fn get_lom_art() {
    let news_api = news::News::new_id(ttr_api::makeclient(),340).unwrap();
    assert_eq!(news_api.get_link(),"https://www.toontownrewritten.com/news/item/340".to_string());
}