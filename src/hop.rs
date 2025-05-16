use std::fmt;

/// Represents a hop addition in a beer recipe.
#[derive(Debug, Clone, PartialEq)]
pub struct Hop {
    pub name: String,
    pub alfa_acid: f32,
    pub grams: f32,
    pub add_time: f32,
    pub original_ibu: f32,
    pub use_value: f32,
    pub scaled_grams: f32,
}

/// Trait for scaling hop additions based on final batch volume.
/// 
/// The scaling formula is:
/// IBU = (w * a * u * 74.89) / v
/// => w = (IBU * V) / (a * u * 74.89)
/// where:
/// - w: weight of hops (oz)
/// - a: alpha acid percentage (decimal)
/// - u: utilization factor (see hop utilization tables, e.g., "How to Brew" by John J. Palmer)
/// - v: volume (gallons)
pub trait ScaleHops {
    fn scale(&self, final_volume: f32) -> f32;
}

/// Creates a new, empty Hop instance.
pub fn new() -> Hop {
    Hop {
        name: String::new(),
        alfa_acid: 0.0,
        grams: 0.0,
        add_time: 0.0,
        original_ibu: 0.0,
        use_value: 0.0,
        scaled_grams: 0.0,
    }
}

impl ScaleHops for Hop {
    fn scale(&self, final_volume: f32) -> f32 {
        // Convert liters to gallons (1 L = 0.264172 gal)
        let volume_gal = final_volume * 0.264172;
        // Calculate hop weight in ounces
        let w_oz = (self.original_ibu * volume_gal) / (self.alfa_acid * self.use_value * 74.89);
        // Convert ounces to grams (1 oz = 28.3495 g)
        w_oz * 28.3495
    }
}

impl fmt::Display for Hop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hop name: {} | Scaled Grams: {:.2} | Add time: {} mins",
            self.name.trim(),
            self.scaled_grams,
            self.add_time
        )
    }
}