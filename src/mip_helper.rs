use binrw::{BinRead, BinWrite,BinReaderExt,BinResult};
use core::matches;

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

impl Mips{
    pub fn generate_levels(w:u16,h:u16,Order:Order) -> Self{
        let direction = Order;
        let mut power = 1;
        let mut level:Vec<Mip>=vec![];
        while((w/power)>0 && (h/power)>0){
            level.push(Mip{resolution:((w/power),(h/power)),img_data:(None)});
            power = power << 1;
        }
        if(matches!(&direction,Order::little)){
            level.reverse();
        }
        Self { direction: (direction), level: (level) }
    }

}