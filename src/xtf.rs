use binrw::{BinRead, BinWrite,BinReaderExt,BinWriterExt,BinResult,io::{Read,Cursor,Write, Seek,SeekFrom}};
use std::io::{BufReader,BufWriter};
use std::path::Path;

use crate::{image_format::{ImageFormat}, mip_helper};



const XTF_MAJOR_VERSION:u32  = 5;
const XTF_MINOR_VERSION:u32  = 0;

#[derive(BinRead, BinWrite)]
pub struct Vector{
    pub x : f32,
    pub y : f32,
    pub z : f32,
}


#[derive(BinRead, BinWrite)]
#[brw(little, magic = b"XTF\x00")]
pub struct XTFHdr {
    pub version:(u32,u32),
    pub header_size: u32,

    pub flags:u32,

    pub width:u16,
    pub height:u16,
    pub depth:u16,

    pub num_frames:u16,
    pub preload_size:u16,
    pub image_data_offset:u16,

    pub reflectivity:Vector,
    pub bump_scale:f32,

    pub image_format:ImageFormat,

    pub low_res_image_width:u8,
    pub low_res_image_height:u8,
    pub fallback_res_image_width:u8,
    pub fallback_res_image_height:u8,

    pub mip_skip_count:u8,
    pub pad:u8,
    
}


pub struct XTFFile {
    pub hdr:XTFHdr,
    pub mips:mip_helper::Mips,
    pub low_res:mip_helper::Mip,
}



impl XTFHdr{
    pub fn open<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        BufReader::new(std::fs::File::open(path)?).read_le()
    }
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        reader.read_le()
    }
    pub fn write<W: Write + Seek>(&self, f: &mut W) -> std::io::Result<()> {
        self.write_le(f);
        f.flush()
    }
    pub fn new() -> Self{
        Self { version: (XTF_MAJOR_VERSION,XTF_MINOR_VERSION), header_size: (58), flags: (0), width: (0), height: (0), depth: (1), num_frames: (1),preload_size:(0), image_data_offset: (0x200), reflectivity: (Vector{ x: (1.0), y: (1.0), z: (1.0) }), bump_scale: (1.0), image_format: (ImageFormat::IMAGE_FORMAT_UNKNOWN), low_res_image_width: (1), low_res_image_height: (1), fallback_res_image_width: (8), fallback_res_image_height: (8), mip_skip_count: (0), pad: (0) }
    }
}
impl XTFFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Self{
        let mut afile = BufReader::new(std::fs::File::open(path).unwrap());
        XTFFile::read(&mut afile)
    }
    pub fn read<R: Read + Seek>(reader: &mut R) -> Self {
        let hdr:XTFHdr = XTFHdr::read(reader).unwrap();
        reader.seek(SeekFrom::Start(hdr.image_data_offset as u64));
        
        let mut mips = mip_helper::Mips::generate_levels(hdr.width.into(), hdr.height.into(), mip_helper::Order::big);
        mips.read_mips(reader, &hdr.image_format);
        let mut mip = mip_helper::Mip{resolution:((hdr.fallback_res_image_width).into(),(hdr.fallback_res_image_height).into()),img_data:(None)};
        mip.read_mip(reader, &hdr.image_format);
        
        XTFFile { hdr: (hdr), mips: (mips), low_res: (mip) }
    }
    pub fn write<W: Write + Seek>(&mut self, f: &mut W) -> std::io::Result<()> {
        self.hdr.write_le(f);
        f.flush();
        f.seek(SeekFrom::Start(self.hdr.image_data_offset as u64));
        f.flush();
        self.mips.write_mips(f);
        f.flush();
        let result = match &self.low_res.img_data {
            Some(data) =>   f.write(&data).unwrap(),
            None => 0,
        };
        f.flush()
    }
}