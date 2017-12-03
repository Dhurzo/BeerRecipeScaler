extern crate argparse;

use malt::ScaleMaltByVolume;
use malt::ScaleMaltByKilo;
use hop::ScaleHops;

mod userinput;
mod recipe;
mod malt;
mod hop;
mod args;


/// Main method, entry point
fn main() {

    println!("Hello, beer!");
    
    let mut hop_number = 0;
    let mut malt_number = 0;
    let mut original_volume = 0.0;
    let mut scaled_volume = 0.0;
    let mut scale_by = "volume".to_string();

    args::parse_args(&mut hop_number, &mut malt_number,
    &mut original_volume,&mut scaled_volume,&mut scale_by);

    args::check_arguments(&malt_number , &original_volume , 
    &scaled_volume , &scale_by );
    
    let mut beer_recipe = recipe::new();
    
    beer_recipe.name = userinput::get_recipe_name();
    beer_recipe.original_volume = original_volume;
    beer_recipe.final_volume = scaled_volume; 
    
    if hop_number > 0 {beer_recipe.hops = userinput::get_hops(hop_number);}
    beer_recipe.malts = userinput::get_malts(malt_number);

    if scale_by == "malt"
    {
        for i in 0..beer_recipe.malts.len()
        {
            beer_recipe.final_volume = beer_recipe.malts[i].
            scale_by_kilo(beer_recipe.original_volume);
        
        }
    }
    else
    {
        for i in 0..beer_recipe.malts.len()
        {
            beer_recipe.malts[i].scaled_kilos= beer_recipe.malts[i].
            scale_by_volumen(beer_recipe.final_volume,beer_recipe.original_volume);
    
        }
               
    }

    for i in 0..beer_recipe.hops.len()
    {
        beer_recipe.hops[i].scaled_grams = beer_recipe.hops[i].scale(beer_recipe.final_volume);
    }

    println!("\n\n\n");
    println!("{}",beer_recipe);
   
}
