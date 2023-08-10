use std::path::Path;
use std::io::BufWriter;

use crate::xtf::*;
use crate::vtf::*;

pub fn xtf2vtf<P: AsRef<Path>>(input:P,output:P){
    let infile = XTFFile::open(input);
    println!("{:?}",infile.hdr.image_format);
    let mut newmips = infile.mips;
    let mut mipsize: u8 = 0;
    for a in &mut newmips{
        a.reverse();
        mipsize = a.level.len() as u8;
    }
    

    let mut outfile = VTFFile{hdr:(VTFHdr { version: ((7,2)), header_size: (0x50), width: (infile.hdr.width), height: (infile.hdr.height), flags: (infile.hdr.flags), num_frames: (infile.hdr.num_frames), start_frame: (0), reflectivity: (VectorAligned { x: (infile.hdr.reflectivity.x), y: (infile.hdr.reflectivity.y), z: (infile.hdr.reflectivity.z) }), bump_scale: (infile.hdr.bump_scale), image_format: (infile.hdr.image_format), num_mip_levels: (mipsize), low_res_image_format: (infile.hdr.image_format), low_res_image_width: (infile.hdr.fallback_res_image_width), low_res_image_height: (infile.hdr.fallback_res_image_height), depth: (infile.hdr.depth),ext:(None)})
    ,mips:(newmips),low_res:(infile.low_res)};
    outfile.write(&mut BufWriter::new(std::fs::File::create(output).unwrap()));
}
pub fn vtf2xtf<P: AsRef<Path>>(input:P,output:P){
    let infile = VTFFile::open(input);
    println!("{:?}",infile.hdr.image_format);
    let mut newmips = infile.mips;
    for a in &mut newmips{
        a.reverse();
    }
    let mut outfile = XTFFile{hdr:(XTFHdr{ version: ((5,0)), header_size: (58),image_data_offset:(0x200),mip_skip_count:(0),pad:(0), width: (infile.hdr.width), height: (infile.hdr.height), flags: (infile.hdr.flags), num_frames: (infile.hdr.num_frames), preload_size: (0), reflectivity: (Vector { x: (infile.hdr.reflectivity.x), y: (infile.hdr.reflectivity.y), z: (infile.hdr.reflectivity.z) }), bump_scale: (infile.hdr.bump_scale), image_format: (infile.hdr.image_format),low_res_image_width:(0),low_res_image_height:(0), fallback_res_image_width: (infile.hdr.low_res_image_width), fallback_res_image_height: (infile.hdr.low_res_image_height), depth: (if infile.hdr.depth>0 {infile.hdr.depth} else {1}) })
    ,mips:(newmips),low_res:(infile.low_res)};
    outfile.write(&mut BufWriter::new(std::fs::File::create(output).unwrap()));
}