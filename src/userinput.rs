use std::io;
use crate::malt;
use crate::hop;

/// Get beer recipe name from user.
///
/// Returns the recipe name as a trimmed String.
pub fn get_recipe_name() -> String {
    let stdin = io::stdin();
    println!("Recipe name: ");
    let mut name = String::new();
    stdin.read_line(&mut name).expect("Failed to read line");
    name.trim().to_string()
}

/// Get a positive float from user input.
///
/// Keeps prompting until a valid positive float is entered.
pub fn get_float() -> f32 {
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f32>() {
            Ok(n) if n >= 0.0 => return n,
            _ => println!("Please use float notation, e.g., 1.5 (must be positive)"),
        }
    }
}

/// Get all recipe malts from user.
///
/// `malts_number`: Number of malts in the recipe.
/// Returns a vector of `malt::Malt`.
pub fn get_malts(malts_number: usize) -> Vec<malt::Malt> {
    let stdin = io::stdin();
    let mut malts = Vec::new();

    for i in 0..malts_number {
        let mut input = String::new();
        let mut malt = malt::new_malt();

        println!("Malt number {}", i + 1);

        println!("Malt name:");
        stdin.read_line(&mut input).expect("Failed to read line");
        malt.name = input.trim().to_string();

        println!("Original KG of malt:");
        malt.original_kilos = get_float();

        println!("Scaled KG of malt (if you are scaling by volume, enter 0):");
        malt.scaled_kilos = get_float();

        malts.push(malt);
    }
    malts
}

/// Get all recipe hops from user.
///
/// `hop_number`: Number of hops in the recipe.
/// Returns a vector of `hop::Hop`.
pub fn get_hops(hop_number: usize) -> Vec<hop::Hop> {
    let stdin = io::stdin();
    let mut hops = Vec::new();

    for i in 0..hop_number {
        let mut input = String::new();
        let mut hop = hop::new();

        println!("Hop number {}\n", i + 1);

        println!("Hop name:");
        stdin.read_line(&mut input).expect("Failed to read line");
        hop.name = input.trim().to_string();

        println!("Hop alpha acids (%):");
        hop.alfa_acid = get_float();

        // Uncomment if you want to ask for grams:
        // println!("Hop grams (fill with 0 if scaling by volume):");
        // hop.grams = get_float();

        println!("Hop IBUs (original recipe):");
        hop.original_ibu = get_float();

        println!("Hop usage (utilization factor):");
        hop.use_value = get_float();

        println!("Hop add time (minutes):");
        hop.add_time = get_float();

        hops.push(hop);
    }
    hops
}