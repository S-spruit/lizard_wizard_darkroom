//i'm a dinosaur!!! rawrs!!!
// anyways, enough joking around, here's the raw metadata
use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};


pub struct RawMetaData {
    pub make: String,
    pub model: String,
    pub black_level: u16,
    pub wb_gains: (f32, f32, f32)
}

pub fn parse_metadata(path: &Path) -> Result<RawMetaData, String> {
    // if is_raw(&path) {
    //     return Err(String::new());
    // }
    let mut rawfile: File = File::open(path).map_err(|e| e.to_string())?;

    let mut header = [0u8; 16];
    rawfile.read_exact(&mut header).map_err(|e| e.to_string())?;
    if &header[0..4] == &[0x49, 0x49, 0x2A, 0x00] {
        println!("Format: TIFF (Little Endian) - CR2, NEF");
    } else if &header[0..4] == &[0x4D, 0x4D, 0x00, 0x2A] {
        println!("Format: TIFF (Big Endian) - Rare for modern RAW");
    } else if &header[0..4] == b"FUJI" {
        println!("Format: Fujifilm RAF");
        // Fuji metadata isn't at the start. You usually have to jump 
        // to a specific offset defined later in the header (often byte 84 or 92)
        // to find the TIFF-like sub-structure.
    } else if &header[4..12] == b"ftypcrx " || &header[4..11] == b"ftypisom" {
        println!("Format: ISO Base Media - CR3");
    } else {
        println!("Unknown format: {:02X?}", &header[0..4]);
        return Err(String::new());
    }
    return  Ok(RawMetaData {
        make: "Canon".to_string(),
        model: "Canon EOS 1200D".to_string(),
        black_level: 1024,
        wb_gains: (1.2, 1.5, 2.0)
    })
}

fn is_raw(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "cr2" | "cr3" | "nef" | "arw" | "raf" | "dng" | "rw2" | "jpg"
        ),
        None => false,
    }
}