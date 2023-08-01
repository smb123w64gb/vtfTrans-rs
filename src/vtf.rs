use binrw::{BinRead, BinWrite,BinReaderExt,BinWriterExt,BinResult, io::{Read,Write,Cursor, Seek,SeekFrom}};
use std::{io::BufReader, sync::Arc};
use std::path::Path;

use crate::{image_format::{ImageFormat,ImageFlags}, mip_helper::{self, Mips}};



const VTF_MAJOR_VERSION:u32  = 7;
const VTF_MINOR_VERSION:u32  = 2;

#[derive(BinRead, BinWrite)]
pub struct VectorAligned{
    #[brw(align_before = 0x10)]
    pub x : f32,
    pub y : f32,
    #[brw(align_after=0x10)]
    pub z : f32,
}


#[derive(BinRead, BinWrite)]
#[brw(little, magic = b"VTF\x00")]
pub struct VTFHdr {
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
    pub depth:u16,
}

pub struct VTFFile{
    pub hdr:VTFHdr,
    pub low_res:mip_helper::Mip,
    pub mips:Vec<mip_helper::Mips>,
}


impl VTFHdr{
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
        Self { version: (VTF_MAJOR_VERSION,VTF_MINOR_VERSION), header_size: (0x50), width: (0), height: (0), flags: (0), num_frames: (1), start_frame: (0), reflectivity: (VectorAligned { x: (1.0), y: (1.0), z: (1.0) }), bump_scale: (1.0), image_format: (ImageFormat::IMAGE_FORMAT_UNKNOWN), num_mip_levels: (1), low_res_image_format: (ImageFormat::IMAGE_FORMAT_UNKNOWN), low_res_image_width: (0), low_res_image_height: (0),depth: (1) }
    }
}

impl VTFFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Self{
        let mut afile = BufReader::new(std::fs::File::open(path).unwrap());
        VTFFile::read(&mut afile)
    }
    pub fn read<R: Read + Seek>(reader: &mut R) -> Self {
        let hdr:VTFHdr = VTFHdr::read(reader).unwrap();
        reader.seek(SeekFrom::Start((hdr.header_size as u64)));

        let mut flags:ImageFlags = ImageFlags::TEXTUREFLAGS_NONE;
        flags.set_to(hdr.flags);
        

        let mut mip = mip_helper::Mip{resolution:((hdr.low_res_image_width).into(),(hdr.low_res_image_height).into()),img_data:(None)};
        if((hdr.low_res_image_format as usize) < (ImageFormat::NUM_IMAGE_FORMATS as usize)){
        mip.read_mip(reader, &hdr.low_res_image_format);
    }
        let mut frames = vec![];
        for i in 0..hdr.num_frames{
        let mut mips = mip_helper::Mips::generate_levels(hdr.width.into(), hdr.height.into(), mip_helper::Order::big,false);

        frames.push(mips);
        };
        for mips in &mut frames{
        mips.read_mips(reader, &hdr.image_format);
    }
        

        VTFFile { hdr: (hdr), mips: (frames), low_res: (mip) }
    }
    pub fn write<W: Write + Seek>(&mut self, f: &mut W) -> std::io::Result<()> {
        self.hdr.write_le(f);
        f.seek(SeekFrom::Start(self.hdr.header_size as u64));
        let result = match &self.low_res.img_data {
            Some(data) =>   f.write(&data).unwrap(),
            None => 0,
        };
        for i in &mut self.mips{
            i.write_mips(f);

    }
    f.flush()
    }
}