mod tests {
    use ttr_api::*;
    #[test]
    fn is_cog_invading() {
        let inv = Invasions::Invasion::new(ttr_api::makeclient().unwrap()).unwrap();
        assert_eq!(inv.cog_invading("Cold Caller"),true) //Change this to a cog that is currently invading.
    }
    #[test]
    fn correct_fo_zone() {
        let walway = 3100;
        assert_eq!(Offices::locale(walway),Some(String::from("Walrus Way")));
    }
    #[test]
    fn invalid_zone() {
        let zone = 2100;
        assert_eq!(Offices::locale(zone),None);
    }
}