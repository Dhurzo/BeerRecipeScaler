use std::io;
use malt;
use hop;

/// Get beer recipe name from user
///
/// Return name : String
pub fn get_recipe_name() -> String
{
    let stdin = io::stdin();
    println!("Recipe name: ");
    let mut name = String::new();
    stdin.read_line(&mut name);
    name
}

/// Get float from user. (force type)
///
/// value : f32
///
fn get_float() -> f32
{   
    let stdin = io::stdin();
    let mut value :f32 = -1.5;
   
    while value < 0.0
    {
        let mut input = String::new();
        stdin.read_line(&mut input);
        value = match input.trim().parse::<f32>() {
            Err(err)  => {println!("Please use float notation ex : 1.5");-1.5},
            Ok(n) => {n},
        };
    }
    
    value
}

/// Get all recipe malts from user
///
/// malts_number : i32  -> Number of malts in the recipe
///
/// Return Vec<malt::Malt> -> Vector of malts
pub fn get_malts(malts_number : i32) -> Vec<malt::Malt>
{
    let stdin = io::stdin();
    let mut malts = Vec::new();
 
    for i in 0..malts_number
    {

        let mut input = String::new();
        let mut float_input : f32 = 0.0;
        let mut malt = malt::new_malt();

        println!("Malt number {}" ,i);

        println!("Malt name :");
        stdin.read_line(&mut input);
        malt.name = input.clone();
        input = "".to_string();

        println!("Original KG of malt : ");
        float_input = get_float();
        malt.original_kilos = float_input.clone();

        println!("Scaled KG of malt: (if you are scaling by volume fill with 0) ");
        float_input = get_float();
        malt.scaled_kilos =  float_input.clone();

        malts.push(malt)
    }
    malts
}

/// Get all recipe hops from user
///
/// hop_number : i32  -> Number of hops  in the recipe
///
/// Return Vec<malt::Hop> -> Vector of hops
pub fn get_hops(hop_number : i32) -> Vec<hop::Hop>
{
    let stdin = io::stdin();
    let mut hops = Vec::new();

    for i in 0..hop_number
    {
        let mut input = String::new();
        let mut float_input = 0.0;
        let mut hop = hop::new();

        println!("Hop number {} \n" ,i);

        println!("Hop name : ");
        stdin.read_line(&mut input);
        hop.name = input;
        input = "".to_string();

        println!("Hop alfa acids : ");
        float_input = get_float();
        hop.alfa_acid = float_input.clone();
        

        /*println!("Hop grams (fill with 0 if scalling by volume) : ");
        float_input = get_float();
        hop.grams = float_input.clone();*/

        println!("Hop ibus (original recipe) : ");
        float_input = get_float();
        hop.original_ibu = float_input.clone();

        println!("Hop usage : ");
        float_input = get_float();
        hop.use_value = float_input.clone();
        
        println!("Hop add time : ");
        float_input = get_float();
        hop.add_time =  float_input.clone();


        hops.push(hop)
    }
    hops
}

