use binrw::{BinReaderExt, BinRead, BinResult};
use std::io::BufReader;
use std::path::Path;
use modular_bitfield::prelude::*;

mod image_format;

#[binrw]
#[br(align_before = 0x10)]
pub struct VectorAligned{
    pub x : f32,
    pub y : f32,
    pub z : f32,
}


#[binrw]
#[brw(little, magic = b"VTF\x00")]
pub struct VTFFile {
    #[brw(count = 2)]
    pub version: Vec<u32>,
    pub header_size: u32,
    pub width:u16,
    pub height:u16,
    pub flags:u32,
    pub num_frames:u16,
    pub start_frame:u16,
    pub reflectivity:VectorAligned,
    pub bump_scale:f32,
    pub image_format:ImageFormat,
    pub num_mip_levels:u8,
    pub low_res_image_format:ImageFormat,
    pub low_res_image_width:u8,
    pub low_res_image_height:u8,
    
   
}



impl VTFFile{
    pub fn open<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        BufReader::new(std::fs::File::open(path)?).read_le()
    }
}