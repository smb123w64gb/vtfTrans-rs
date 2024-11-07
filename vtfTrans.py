import struct
from enum import IntFlag
import swizzle

def u8(file):
    return struct.unpack("B", file.read(1))[0]
 
def u16(file):
    return struct.unpack("<H", file.read(2))[0]
 
def u32(file):
    return struct.unpack("<I", file.read(4))[0]

def s32(file):
    return struct.unpack("<i", file.read(4))[0]

def f32(file):
    return struct.unpack("f", file.read(4))[0]

def padvec3(file):
    return struct.unpack("fffff", file.read(0x14))[1:4]
def vec3(file):
    return struct.unpack("fff", file.read(12))[0:3]

def w32(file,val):
    file.write(struct.pack("<I", val))
def wb32(file,val):
    file.write(struct.pack(">I", val))    

def ws32(file,val):
    file.write(struct.pack("<i", val))

def w16(file,val):
    file.write(struct.pack("<H", val))

def ws16(file,val):
    file.write(struct.pack("<h", val))

def w8(file,val):
    file.write(struct.pack("B", val))

def ws8(file,val):
    file.write(struct.pack("b", val))

def wstr(file,val):
    file.write(struct.pack("s", val))
def wvec3(file,vec):
    file.write(struct.pack("fff", vec[0],vec[1],vec[2]))

def wf32(file,val):
    file.write(struct.pack("f", val))


def getstr(f, term=b'\0'):
    result = ""
    tmpChar = f.read(1).decode("ASCII")
    while ord(tmpChar) != 0:
        result += tmpChar
        tmpChar = f.read(1).decode("ASCII")
    return result

def rS(f,o):
    c = f.tell()
    f.seek(o)
    s = getstr(f)
    f.seek(c)
    return s

def rR(f,o,l):
    c = f.tell()
    f.seek(o)
    d = f.read(l)
    f.seek(c)
    return d

ImageFormat = [
    ["IMAGE_FORMAT_RGBA8888",4], #0
	["IMAGE_FORMAT_ABGR8888",4], #1
	["IMAGE_FORMAT_RGB888",3] ,#2
	["IMAGE_FORMAT_BGR888",3],#3
	["IMAGE_FORMAT_RGB565",2],#4
	["IMAGE_FORMAT_I8",1],#5
	["IMAGE_FORMAT_IA88",2],#6
	["IMAGE_FORMAT_P8",1],#7
	["IMAGE_FORMAT_A8",1],#8
	["IMAGE_FORMAT_RGB888_BLUESCREEN",3],#9
	["IMAGE_FORMAT_BGR888_BLUESCREEN",3],#10
	["IMAGE_FORMAT_ARGB8888",4],#11
	["IMAGE_FORMAT_BGRA8888",4],#12
	["IMAGE_FORMAT_DXT1",8],#13
	["IMAGE_FORMAT_DXT3",16],#14
	["IMAGE_FORMAT_DXT5",16],#15
	["IMAGE_FORMAT_BGRX8888",4],#16
	["IMAGE_FORMAT_BGR565",2],#17
	["IMAGE_FORMAT_BGRX5551",2],#18
	["IMAGE_FORMAT_BGRA4444",2],#19
	["IMAGE_FORMAT_DXT1_ONEBITALPHA",8],#20
	["IMAGE_FORMAT_BGRA5551",2],#21
	["IMAGE_FORMAT_UV88",2],#22
	["IMAGE_FORMAT_UVWQ8888",4],#23
	["IMAGE_FORMAT_RGBA16161616F",8],#24
	["IMAGE_FORMAT_RGBA16161616",8],#25
	["IMAGE_FORMAT_UVLX8888",4],#26
	["IMAGE_FORMAT_R32F",4],#27
	["IMAGE_FORMAT_RGB323232F",12],#28
	["IMAGE_FORMAT_RGBA32323232F",16],#29
    ["IMAGE_FORMAT_LINEAR_BGRX8888",4],#30
	["IMAGE_FORMAT_LINEAR_RGBA8888",4],#31
	["IMAGE_FORMAT_LINEAR_ABGR8888",4],#32
	["IMAGE_FORMAT_LINEAR_ARGB8888",4],#33
	["IMAGE_FORMAT_LINEAR_BGRA8888",4],#34
	["IMAGE_FORMAT_LINEAR_RGB888",3],#35
	["IMAGE_FORMAT_LINEAR_BGR888",3],#36
	["IMAGE_FORMAT_LINEAR_BGRX5551",2],#37
	["IMAGE_FORMAT_LINEAR_I8",1],#38
    ]




class TextureFlags(IntFlag):
    TEXTUREFLAGS_POINTSAMPLE	               = 0x00000001
    TEXTUREFLAGS_TRILINEAR		               = 0x00000002
    TEXTUREFLAGS_CLAMPS			               = 0x00000004
    TEXTUREFLAGS_CLAMPT			               = 0x00000008
    TEXTUREFLAGS_ANISOTROPIC	               = 0x00000010
    TEXTUREFLAGS_HINT_DXT5		               = 0x00000020
    TEXTUREFLAGS_NOCOMPRESS		               = 0x00000040
    TEXTUREFLAGS_NORMAL			               = 0x00000080
    TEXTUREFLAGS_NOMIP			               = 0x00000100
    TEXTUREFLAGS_NOLOD			               = 0x00000200
    TEXTUREFLAGS_MINMIP			               = 0x00000400
    TEXTUREFLAGS_PROCEDURAL		               = 0x00000800
    TEXTUREFLAGS_ONEBITALPHA	               = 0x00001000
    TEXTUREFLAGS_EIGHTBITALPHA	               = 0x00002000
    TEXTUREFLAGS_ENVMAP			               = 0x00004000
    TEXTUREFLAGS_RENDERTARGET	               = 0x00008000
    TEXTUREFLAGS_DEPTHRENDERTARGET	           = 0x00010000
    TEXTUREFLAGS_NODEBUGOVERRIDE               = 0x00020000
    TEXTUREFLAGS_SINGLECOPY		               = 0x00040000
    TEXTUREFLAGS_ONEOVERMIPLEVELINALPHA        = 0x00080000
    TEXTUREFLAGS_PREMULTCOLORBYONEOVERMIPLEVEL = 0x00100000
    TEXTUREFLAGS_NORMALTODUDV                  = 0x00200000
    TEXTUREFLAGS_ALPHATESTMIPGENERATION        = 0x00400000
    TEXTUREFLAGS_NODEPTHBUFFER                 = 0x00800000
    TEXTUREFLAGS_NICEFILTERED                  = 0x01000000
    TEXTUREFLAGS_CLAMPU                        = 0x02000000
    TEXTUREFLAGS_PRESWIZZLED                   = 0x04000000
    TEXTUREFLAGS_CACHEABLE                     = 0x08000000
    TEXTUREFLAGS_UNFILTERABLE_OK               = 0x10000000

class VTF(object):
    def __init__(self):
        self.fileTypeString = 'VTF\x00'
        self.versionhi = 7
        self.versionlo = 2
        self.headersize = 0x40
        self.width = 0
        self.height = 0 
        self.flags = TextureFlags(0)
        self.numFrames = 0
        self.startFrame = 0
        self.reflectivity = [0.0,0.0,0.0] #There is padding front and back
        self.bumpScale = 0.0
        self.imageFormat = 0
        self.numMipLevels = 1
        self.lowResImageFormat = 0
        self.lowResImageWidth = 0
        self.lowResImageHeight = 0
        self.depth = 0

    def read(self,f):
        self.fileTypeString = f.read(4)
        self.versionhi = u32(f)
        self.versionlo = u32(f)
        self.headersize = u32(f)
        self.width = u16(f)
        self.height = u16(f) 
        self.flags = TextureFlags(u32(f))
        self.numFrames = u16(f)
        self.startFrame = u16(f)
        self.reflectivity = padvec3(f)
        self.bumpScale = f32(f)
        self.imageFormat = u32(f)
        self.numMipLevels = u8(f)
        self.lowResImageFormat = u32(f)
        self.lowResImageWidth = u8(f)
        self.lowResImageHeight = u8(f)
        self.depth = u16(f)
    def write(self,f):
        f.write(bytes(self.fileTypeString,"utf-8"))
        w32(f,self.versionhi)
        w32(f,self.versionlo)
        w32(f,self.headersize)
        w16(f,self.width)
        w16(f,self.height) 
        w32(f,int(int(self.flags)))
        
        w16(f,self.numFrames)
        w16(f,self.startFrame)

        w32(f,0) #pad
        wvec3(f,self.reflectivity)
        w32(f,0) #pad
        wf32(f,self.bumpScale)
        w32(f,self.imageFormat)
        w8(f,self.numMipLevels)
        w32(f,self.imageFormat)
        w8(f,self.lowResImageWidth)
        w8(f,self.lowResImageHeight)
        w16(f,1)

class XTF(object):
    def __init__(self):
        self.fileTypeString = 'XTF\x00'
        self.versionhi = 5
        self.versionlo = 0
        self.headersize = 58
        self.flags = TextureFlags(0)
        self.width = 0
        self.height = 0 
        self.depth = 0
        self.numFrames = 0
        self.preloadDataSize = 0
        self.imageDataOffset = 0x200

        self.reflectivity = [1.0,1.0,1.0]
        self.bumpScale = 1.0
        self.imageFormat = 0

        self.lowResImageWidth = 1
        self.lowResImageHeight = 1

        self.fallbackImageWidth = 1
        self.fallbackImageHeight = 1

        self.mipSkipCount = 1
        self.pad = 0
    def read(self,f):
        self.fileTypeString = f.read(4)
        self.versionhi = u32(f)
        self.versionlo = u32(f)
        self.headersize = u32(f)
        self.flags = TextureFlags(u32(f))
        self.width = u16(f)
        self.height = u16(f) 
        self.depth = u16(f)
        self.numFrames = u16(f)
        self.preloadDataSize = u16(f)
        self.imageDataOffset = u16(f)

        self.reflectivity = vec3(f)
        self.bumpScale = f32(f)
        self.imageFormat = u32(f)

        self.lowResImageWidth = u8(f)
        self.lowResImageHeight = u8(f)
        self.fallbackImageWidth = u8(f)
        self.fallbackImageHeight = u8(f)

        self.mipSkipCount = u8(f)
        self.pad = u8(f)
    def write(self,f):
        f.write(bytes(self.fileTypeString,"utf-8"))
        w32(f,self.versionhi)
        w32(f,self.versionlo)
        w32(f,self.headersize)
        w32(f,int(int(self.flags)|TextureFlags.TEXTUREFLAGS_PRESWIZZLED))
        w16(f,self.width)
        w16(f,self.height) 
        w16(f,self.depth)
        w16(f,self.numFrames)
        w16(f,self.preloadDataSize)
        w16(f,self.imageDataOffset)

        wvec3(f,self.reflectivity)
        wf32(f,self.bumpScale)
        w32(f,self.imageFormat)

        w8(f,self.lowResImageWidth)
        w8(f,self.lowResImageHeight)
        w8(f,self.fallbackImageWidth)
        w8(f,self.fallbackImageHeight)
        w8(f,self.mipSkipCount)
        w8(f,self.pad)



#DTX1/BC1 = 8 bytes pre chunk for 4x4/16px .5 size
#DTX3|5/BC2|3 16 bytes per chink for 4x4/16px 1 to 1

def blockSize(f):
    if(f == 13 or f==20):
        return 8
    elif(f==14 or f==15):
        return 16
    else:
        return 1
def getmipssize(w,h,f):#2 is for BC1
        Block_Size = w*h

        minsize = blockSize(f)
        blockArray = []
        sizeArray = getRes(w,h)
        for a in sizeArray:
            x = a[0]
            y = a[1]
            if(minsize>1):
                if(x<4):
                    x = 4
                if(y<4):
                    y = 4
                blockArray.append(int(((x*y)/2 if minsize==8 else 1)))
            else:
                blockArray.append(x*y*ImageFormat[f][1])
        return blockArray
def getRes(w,h):
    res = []
    i = 1
    while((w/i)>=1 and (w/i)>=1):
        res.append([int(w/i),int(h/i)])
        i = i<<1
    return res

import sys,os

if(len(sys.argv) > 2):
    isXbox = False
    ext = ['.vtf','.xtf']
    inType = sys.argv[1].lower()
    if(inType == "-xtf"):
        isXbox = False
        inFormat = VTF()
        outFormat = XTF()
        
    elif(inType == "-vtf"):
        isXbox = True
        inFormat = XTF()
        outFormat = VTF()
        ext.reverse()

    inFile = open(sys.argv[2], "rb")
    outFile = open(sys.argv[2].replace(ext[0],ext[1]), "wb")
        
    inFormat.read(inFile)

    print(ImageFormat[inFormat.imageFormat])
    outFormat.bumpScale = inFormat.bumpScale
    outFormat.depth = inFormat.depth
    outFormat.flags = inFormat.flags
    outFormat.height = inFormat.height
    outFormat.width = inFormat.width
    outFormat.numFrames = inFormat.numFrames
    outFormat.reflectivity = inFormat.reflectivity
    outFormat.imageFormat = inFormat.imageFormat
    if(isXbox):
        outFormat.lowResImageFormat = inFormat.imageFormat
        outFormat.lowResImageWidth = inFormat.fallbackImageWidth
        outFormat.lowResImageHeight = inFormat.fallbackImageHeight
    else:
        outFormat.fallbackImageWidth = inFormat.lowResImageWidth
        outFormat.fallbackImageHeight = inFormat.lowResImageHeight

    bs = getmipssize(inFormat.width,inFormat.height,inFormat.imageFormat)
    rs = getRes(inFormat.width,inFormat.height)
    if(isXbox):
        outFormat.numMipLevels = len(bs)
        print(bs)
    bs_sec = []
    dim = inFormat.fallbackImageWidth * inFormat.fallbackImageHeight if isXbox else inFormat.lowResImageWidth * inFormat.lowResImageHeight
    print(dim)
    fallbackSize = int(dim /(2 if blockSize(outFormat.imageFormat) == 8  else 1))
    if(blockSize(outFormat.imageFormat)==1):
        fallbackSize *= ImageFormat[outFormat.imageFormat][1]
    print(fallbackSize)

    
    off = inFormat.imageDataOffset if isXbox else inFormat.headersize
    inFile.seek(off)
    if(not isXbox):
        bs.reverse()
        bs_sec.append(inFile.read(fallbackSize))
        rs.append([inFormat.lowResImageWidth,inFormat.lowResImageHeight])
    for x in bs:
        for y in range(inFormat.numFrames):
            bs_sec.append(inFile.read(x))
    if(isXbox):
        bs_sec.append(inFile.read(fallbackSize))
        rs.append([inFormat.fallbackImageWidth,inFormat.fallbackImageHeight])

    bsS_sec = []
    if(blockSize(inFormat.imageFormat) <1):
        for x in range(len(bs_sec)):
            print(x)
            if(isXbox):
                bsS_sec.append(swizzle.unswizzle_rect(bs_sec[x],rs[x][0],rs[x][1],rs[x][0]*ImageFormat[inFormat.imageFormat][1],ImageFormat[inFormat.imageFormat][1]))
            else:
                bsS_sec.append(swizzle.swizzle_rect(bs_sec[x],rs[x][0],rs[x][1],rs[x][0]*ImageFormat[inFormat.imageFormat][1],ImageFormat[inFormat.imageFormat][1]))
    else:
        bsS_sec = bs_sec
    outFormat.write(outFile)
    off = outFormat.imageDataOffset if not isXbox else outFormat.headersize
    outFile.seek(off)
    bsS_sec.reverse()
    for x in bsS_sec:
        outFile.write(x)




    outFile.close()
    inFile.close()
else:
    print("Command line: vtfTrans.py\n")
    print("usage\t: vtfTrans.py -xtf texture.vtf <optional.xtf>\n")
    print("Common options:")
    print("\t-xtf\t: Transition vtf to xtf")
    print("\t-vtf\t: Transition xtf to vtf")