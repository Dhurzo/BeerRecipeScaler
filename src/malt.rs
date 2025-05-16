use std::fmt;

/// Represents a malt addition in a beer recipe.
#[derive(Debug, Clone, PartialEq)]
pub struct Malt {
    pub name: String,
    pub original_kilos: f32,
    pub scaled_kilos: f32,
}

/// Creates a new, empty Malt instance.
pub fn new_malt() -> Malt {
    Malt {
        name: String::new(),
        original_kilos: 0.0,
        scaled_kilos: 0.0,
    }
}

/// Trait for scaling malt by volume (proportional scaling).
pub trait ScaleMaltByVolume {
    /// Scales malt (kg) using proportional scale.
    fn scale_by_volume(&self, final_volume: f32, original_volume: f32) -> f32;
}

/// Trait for scaling malt by kilo.
pub trait ScaleMaltByKilo {
    fn scale_by_kilo(&self, original_volume: f32) -> f32;
}

impl ScaleMaltByVolume for Malt {
    fn scale_by_volume(&self, final_volume: f32, original_volume: f32) -> f32 {
        (final_volume * self.original_kilos) / original_volume
    }
}

impl ScaleMaltByKilo for Malt {
    fn scale_by_kilo(&self, original_volume: f32) -> f32 {
        (self.scaled_kilos * original_volume) / self.original_kilos
    }
}

impl fmt::Display for Malt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Malt Name: {}  ====  {:.2} scaled KG\n",
            self.name.trim(),
            self.scaled_kilos
        )
    }
}