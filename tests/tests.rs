mod tests {
    use ttr_api::*;

    #[test]
    #[ignore]
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
    #[test]
    fn doodle_render() {
        let dod = Doodles::Render::make("64ff02ff01030e010001",256,"png");
        assert_eq!(dod.render(),String::from("rendition.toontownrewritten.com/render/64ff02ff01030e010001/doodle/256x256.png"));
    }
}