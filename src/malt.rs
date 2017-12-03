use std::fmt;

pub struct Malt
{
    pub name: String,
    pub original_kilos: f32,
    pub scaled_kilos: f32,
}

pub fn new_malt() -> Malt
{
    Malt
    {
            name: "".to_string(),
            original_kilos: 0.0,
            scaled_kilos: 0.0,
    }
}

pub trait ScaleMaltByVolume 
{
    //scale malt KG (metric system) using proportional scale
    fn scale_by_volumen(&self,finalVolume:f32,originalVolume:f32) -> f32;
}

pub trait ScaleMaltByKilo
{
    fn scale_by_kilo(&self,originalVolume:f32) -> f32;
}

impl ScaleMaltByVolume for Malt
{
    fn scale_by_volumen(&self,final_volume:f32,original_volume:f32) -> f32 
    {
         (final_volume*self.original_kilos)/original_volume
    }        
}

impl ScaleMaltByKilo for Malt
{
    
    fn scale_by_kilo(&self,original_volume:f32) -> f32
    {
        (self.scaled_kilos*original_volume)/self.original_kilos
    }
}

impl fmt::Display for Malt
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f , "Malt Name: {}  ====  {} scaled KG \n",
         self.name.trim(),self.scaled_kilos)
    }
}       
