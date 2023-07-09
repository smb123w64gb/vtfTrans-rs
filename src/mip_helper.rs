use core::matches;
use std::io::{BufReader,BufWriter,Write,Read};
use std::path::Path;
pub struct Mip{
    pub resolution : (u16,u16),
    pub img_data : Option<Vec<u8>>,
}
pub enum Order{
    little,
    big,
}

pub struct Mips{
    pub direction : Order,
    pub level : Vec<Mip>,
}


impl Mip {
    pub fn read_mip<R: Read>(&mut self,mut reader: R,bpp:i32){
        let mut data = vec![0u8;((self.resolution.0 * self.resolution.1) as i32 *bpp) as usize];
        reader.read_exact(&mut data);
        self.img_data = Some(data)
    }
    
}

impl Mips{
    pub fn generate_levels(w:u16,h:u16,Order:Order) -> Self{
        let direction = Order;
        let mut power = 1;
        let mut level:Vec<Mip>=vec![];
        while(w/power)>0 && (h/power)>0{
            level.push(Mip{resolution:((w/power),(h/power)),img_data:(None)});
            power = power << 1;
        }
        if matches!(&direction,Order::little) {
            level.reverse();
        }
        Self { direction: (direction), level: (level) }
    }
    pub fn reverse(&mut self){
        match self.direction {
            Order::big => self.direction = Order::little,
            Order::little => self.direction = Order::big,
        }
        self.level.reverse();

    }
    pub fn read_mips<R: Read>(&mut self,mut reader: R,bpp:i32){
        for mut a in &mut self.level{
            a.read_mip(&mut reader, bpp);
        }
    }

}