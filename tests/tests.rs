mod tests {
    use ttr_api::*;

    #[test]
    #[ignore]
    fn is_cog_invading() {
        let inv = Invasions::Invasion::new(ttr_api::makeclient()).unwrap();
        let test = inv.cog_invading("Cold Caller"); //Change this to a cog that is currently invading.
        assert_eq!(test.0,true)
    }
    #[test]
    fn is_cog_not_invading() {
        let inv = Invasions::Invasion::new(ttr_api::makeclient()).unwrap();
        let test = inv.cog_invading("Director of Ambush Marketing");
        assert_eq!(test.0,false)
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
        let doodle = Doodles::Doodle {dna:String::from("64ff02ff01030e010001"),traits:vec!(String::from("godawful")),cost:1};
        let dod = Doodles::Doodle::render(&doodle,256,"png");
        assert_eq!(dod,String::from("rendition.toontownrewritten.com/render/64ff02ff01030e010001/doodle/256x256.png"));
    }
    #[test]
    #[ignore]
    fn real_doodle_render() {
        let doodle_api = Doodles::Districts::new(ttr_api::makeclient()).unwrap();
        let doodle = doodle_api.get_doodle("Blam Canyon","Toontown Central",0).unwrap();
        assert_eq!(doodle.render(256,"png"),String::from("rendition.toontownrewritten.com/render/64000302010102050001/doodle/256x256.png")) //Needs to be a real doodle in Blam Canyon, Toontown Central on the day it's tested.
    }
}