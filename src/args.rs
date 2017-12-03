extern crate argparse;
use argparse::{ArgumentParser,Store};

/// Checks legality of arguments.
pub fn check_arguments(malt_number : &i32, original_volume : &f32 , scaled_volume : &f32, scale_by : &String)
{
    if *malt_number <= 0 { panic!("No malts no recipe")};
    if *original_volume <= 0.0 {panic!("No volume no recipe")}
    if *scaled_volume <= 0.0 && scale_by.to_string() != "malt"  {panic!("No volume with scale by volume no recipe")}
}

/// Parse args
pub fn parse_args( hop_number: &mut i32, malt_number:  &mut i32, original_volume: &mut f32, scaled_volume :  &mut f32, scale_by:  &mut String)
{
        {
        let mut ap = ArgumentParser::new();
        
        ap.set_description("Scale beer recipe (malts and hops for now)");

        ap.refer( hop_number)
            .add_option(&["-l","--hops"],Store,"Number of hop aditions");
        
        ap.refer(malt_number)
            .add_option(&["-m","--malts"],Store,"Numer of malts");
        
        ap.refer(original_volume)
            .add_option(&["-o","--originalVolume"],Store,"Original recipe volume");
        
        ap.refer(scaled_volume)
            .add_option(&["-s","--scaledVolume"],Store,"Scaled recipe volume / malt");

        ap.refer(scale_by)
            .add_option(&["-b","--scaleby"],Store,"Scale by malt (malt) or by final volume (volume)");
        
        ap.parse_args_or_exit();
    }

}