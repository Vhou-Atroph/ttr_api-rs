fn main() {
    ttrpopcheck_rs::get_json(ttrpopcheck_rs::makeclient().unwrap()).unwrap();
}