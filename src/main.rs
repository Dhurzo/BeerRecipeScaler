use clap::Parser;
use malt::{ScaleMaltByVolume, ScaleMaltByKilo};
use hop::ScaleHops;

mod userinput;
mod recipe;
mod malt;
mod hop;
mod args;

fn main() {
    println!("Hello, beer!");

    let args = args::Args::parse();
    args::check_arguments(&args);

    let mut beer_recipe = recipe::new();

    beer_recipe.name = userinput::get_recipe_name();
    beer_recipe.original_volume = args.original_volume;
    beer_recipe.final_volume = args.scaled_volume;

    if args.hop_number > 0 {
        beer_recipe.hops = userinput::get_hops(args.hop_number);
    }
    beer_recipe.malts = userinput::get_malts(args.malt_number);

    match args.scale_by.as_str() {
        "malt" => {
            for malt in beer_recipe.malts.iter_mut() {
                beer_recipe.final_volume = malt.scale_by_kilo(beer_recipe.original_volume);
            }
        }
        _ => {
            for malt in beer_recipe.malts.iter_mut() {
                malt.scaled_kilos = malt.scale_by_volume(
                    beer_recipe.final_volume,
                    beer_recipe.original_volume,
                );
            }
        }
    }

    for hop in beer_recipe.hops.iter_mut() {
        hop.scaled_grams = hop.scale(beer_recipe.final_volume);
    }

    println!("\n\n\n");
    println!("{}", beer_recipe);
}