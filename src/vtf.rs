use binrw::{BinRead, BinWrite,BinReaderExt,BinResult};
use std::io::BufReader;
use std::path::Path;

use crate::image_format::ImageFormat;

#[derive(BinRead, BinWrite)]

pub struct VectorAligned{
    #[br(align_before = 0x10)]
    pub x : f32,
    pub y : f32,
    #[br(align_after=0x10)]
    pub z : f32,
}


#[derive(BinRead, BinWrite)]
#[brw(little, magic = b"VTF\x00")]
pub struct VTFFile {
    pub version:(u32,u32),
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