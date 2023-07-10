fn generate_swizzle_masks(width:u32 ,height:u32 ,depth:u32 ,
                        mut mask_x:u32 ,mut mask_y:u32 ,mut mask_z:u32){
    let mut x:u32 = 0;
    let mut y:u32 = 0;
    let mut z:u32 = 0;

    let mut bit:u32 = 1;
    let mut mask_bit:u32 = 1;

    let mut done:bool;
    loop{
        done = true;
        if (bit < width) { x |= mask_bit; mask_bit <<= 1; done = false; }
        if (bit < height) { y |= mask_bit; mask_bit <<= 1; done = false; }
        if (bit < depth) { z |= mask_bit; mask_bit <<= 1; done = false; }
        bit <<= 1;
        if(done){break;}
    }
    assert!(((x ^ y) ^ z) == (mask_bit - 1));
    mask_x = x;
    mask_y = y;
    mask_z = z;

}
fn fill_pattern(pattern:u32,value:u32)-> u32{
    let mut value = value;
    let mut result:u32 = 0;
    let mut bit:u32 = 0;
    while(value>0){
        if ((pattern & bit)>0) {
            /* Copy bit to result */
            result |= if (value & 1)>0 {bit}else{0};
            value >>= 1;
        }
        bit <<= 1;
    }
    result
}

fn get_swizzled_offset(x:u32,y:u32,z:u32,
    mask_x:u32,mask_y:u32,mask_z:u32,
    bytes_per_pixel:u32) -> u32{
        bytes_per_pixel * (fill_pattern(mask_x, x)
                           | fill_pattern(mask_y, y)
                           | fill_pattern(mask_z, z))
}

