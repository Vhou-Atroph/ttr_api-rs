use ttr_api::Population;
use ttr_api::SillyMeter;
use ttr_api::Invasions;
use ttr_api::Offices;
fn main() {
    let pop = Population::PopAPI::new(ttr_api::makeclient().unwrap()).unwrap();
    println!("{:?}",pop);
    let met = SillyMeter::Meter::new(ttr_api::makeclient().unwrap()).unwrap();
    println!("{:?}",met);
    let inv = Invasions::Invasion::new(ttr_api::makeclient().unwrap()).unwrap();
    println!("{:?}",inv);
    let off = Offices::Office::new(ttr_api::makeclient().unwrap()).unwrap();
    println!("{:?}",off);
}