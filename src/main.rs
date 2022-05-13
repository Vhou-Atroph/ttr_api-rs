use ttr_api_rs::Population;
fn main() {
    println!("{}",Population::pop_info(&Population::pop_api(ttr_api_rs::makeclient().unwrap()).unwrap()));
}