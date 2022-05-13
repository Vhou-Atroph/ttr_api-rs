use ttr_api_rs::Population;
use ttr_api_rs::SillyMeter;
use ttr_api_rs::Invasions;
fn main() {
    let pop = Population::PopAPI::new(ttr_api_rs::makeclient().unwrap()).unwrap();
    println!("{:?}",pop);
    let met = SillyMeter::Meter::new(ttr_api_rs::makeclient().unwrap()).unwrap();
    println!("{:?}",met);
    let inv = Invasions::Invasion::new(ttr_api_rs::makeclient().unwrap()).unwrap();
    println!("{:?}",inv);
}