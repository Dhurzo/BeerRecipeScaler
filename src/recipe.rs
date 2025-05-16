use std::fmt;
use crate::hop::Hop;
use crate::malt::Malt;

/// Represents a beer recipe.
#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub original_volume: f32,
    pub final_volume: f32,
    pub hops: Vec<Hop>,
    pub malts: Vec<Malt>,
}

/// Creates a new, empty Recipe instance.
pub fn new() -> Recipe {
    Recipe {
        name: String::new(),
        original_volume: 0.0,
        final_volume: 0.0,
        hops: Vec::new(),
        malts: Vec::new(),
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Recipe name: {}  Recipe scaled volume: {} Liters\n",
            self.name.trim(),
            self.final_volume
        )?;

        writeln!(f, "======= MALTS =======")?;
        for m in &self.malts {
            writeln!(f, "{}", m)?;
        }

        writeln!(f, "======= HOPS =======")?;
        for h in &self.hops {
            writeln!(f, "{}", h)?;
        }

        write!(f, "===================")
    }
}