use std::fmt;
use hop::Hop;
use malt::Malt;

/// Strut represents a beer recipe
pub struct Recipe{
    pub  name : String,
    pub  original_volume: f32,
    pub  final_volume : f32,
    pub  hops : Vec<Hop>,
    pub  malts : Vec<Malt>
}

/// Instance new empty recipe
pub fn new() -> Recipe
{
    Recipe {    
        name : "".to_string(),
        original_volume: 0.0,
        final_volume : 0.0,
        hops : Vec::new(),
        malts : Vec::new()
    }
}
 
impl fmt::Display for Recipe{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f , "Recipe name :{}  Recipe scaled volume: {} Liters \n\n",self.name.trim(),self.final_volume);
                
        println!("=======MALTS======= \n\n");
        for m in &self.malts{println!("{}",m);}

        println!("=======HOPS======= \n\n");
        for h in &self.hops{println!("{}",h);}

        write!(f,"===================")
    }

}
