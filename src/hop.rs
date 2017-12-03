use std::fmt;

/// Strut represents hop
pub struct Hop
{
    pub name: String,
    pub alfa_acid: f32,
    pub grams: f32,
    pub add_time: f32,
    pub original_ibu: f32,
    pub use_value:f32,
    pub scaled_grams: f32,
}


///scale hop using formula: IBU = (w*a*u*74.89)/v ====> w = (IBU*V) / (a*u*74.89)
///u it's usage value, you have to check a hop usage table to find out the value
///you can check one of this table in the book "How to brew - John J. Palmer"
pub trait ScaleHops
{

    fn scale(&self,finalVolume:f32) -> f32;
}


/// Instances new empty hop
pub fn new() -> Hop
{
    Hop
    {
            name: "".to_string(),
            alfa_acid: 0.0,
            grams: 0.0,
            add_time: 0.0,
            original_ibu: 0.0,
            use_value:0.0,
            scaled_grams: 0.0,
    }
}
///
/// Implementation of ScaleHops
/// 
impl ScaleHops for Hop
{

    fn scale(&self,final_volume:f32) -> f32 
    {
        let w:f32 = (self.original_ibu*(final_volume*0.264172)) / (self.alfa_acid*self.use_value*74.89); //0,264172 liters to gallons "cast"
        w*28.3495 // oz to grams "cast"
    }

}


impl fmt::Display for Hop
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f , "Hop name {}  ==== Scaled Grams: {} Add time: ==== {} mins \n",
               self.name.trim(),self.scaled_grams,self.add_time)
    }
}
