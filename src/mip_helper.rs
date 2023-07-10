use core::matches;
use std::io::{BufReader,BufWriter,Write,Read};
use std::path::Path;

use crate::image_format;
pub struct Mip{
    pub resolution : (usize,usize),
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
    pub fn read_mip<R: Read>(&mut self,mut reader:  &mut R,format : &image_format::ImageFormat){
        println!("W:{}\tH:{}",self.resolution.0,self.resolution.1);
        let w = if self.resolution.0%4 == 0{self.resolution.0}  else {(self.resolution.0+(4-(self.resolution.0 % 4)))};
        let h = if self.resolution.1%4 == 0{self.resolution.1}  else {(self.resolution.1+(4-(self.resolution.1 % 4)))};
        let size = match format {
            image_format::ImageFormat::IMAGE_FORMAT_DXT1 => (w*h)>>1,
            image_format::ImageFormat::IMAGE_FORMAT_DXT1_ONEBITALPHA => (w*h)>>1,
            image_format::ImageFormat::IMAGE_FORMAT_DXT3 => (w*h),
            image_format::ImageFormat::IMAGE_FORMAT_DXT5 => (w*h),
            _ => (((self.resolution.0 * self.resolution.1) as usize)* image_format::ImageFormatBlock[*format as usize]),
            
        };
        println!("{}",size);
        let mut data = vec![0u8;size];
        reader.read_exact(&mut data);
        self.img_data = Some(data)
    }
    
}

impl Mips{
    pub fn generate_levels(w:usize,h:usize,Order:Order) -> Self{
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
    pub fn read_mips<R: Read>(&mut self,mut reader:  &mut R,format : &image_format::ImageFormat){
        for a in &mut self.level{
            a.read_mip(&mut reader, format);
        }
    }
    pub fn write_mips<W: Write>(&mut self, f: &mut W) -> std::io::Result<()>{
        for a in &mut self.level{
            let result = match &a.img_data {
                Some(data) => f.write(&data).unwrap(),
                None => 0,
            };
        }
        f.flush()
    }

}