use ttr_api_rs::Population;
fn main() {
    let pop = Population::PopAPI::new(ttr_api_rs::makeclient().unwrap()).unwrap();
    println!("{}",Population::pop_info(&pop));
}