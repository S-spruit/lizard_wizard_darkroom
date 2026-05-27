//i'm a dinosaur!!! rawrs!!!
// anyways, enough joking around, here's the raw metadata
use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

//structs
#[derive(Clone, Copy)]
pub struct ExifData {
    pub iso: u32,
    pub exposure_time: u32,
    pub makernote_offset: u32,
    pub wb_gains: (f32, f32, f32),
}

pub struct RawMetaData {
    pub make: String,
    pub model: String,
    pub black_level: u16,
    pub wb_gains: (f32, f32, f32),
    pub exif: Option<ExifData>
}

struct TiffTag {
    id: u16,
    data_type: u16,
    count: u32,
    offset: u32
}
// parsing

pub fn parse_metadata(path: &Path) -> Result<RawMetaData, String> {
    let mut rawfile: File = File::open(path).map_err(|e| e.to_string())?;
    let mut header = [0u8; 16];
    rawfile.read_exact(&mut header).map_err(|e| e.to_string())?;

    let mut meta = RawMetaData {
        make: String::new(),
        model: String::new(),
        black_level: 2048,
        wb_gains: (1.0, 1.0, 1.0),
        exif: None,
    };

    if &header[0..4] == &[0x49, 0x49, 0x2A, 0x00] {
        let ifd0_offset = u32::from_le_bytes(header[4..8].try_into().unwrap());
        
        rawfile.seek(SeekFrom::Start(ifd0_offset as u64)).map_err(|e| e.to_string())?;

        let mut count_buf = [0u8; 2];
        rawfile.read_exact(&mut count_buf).map_err(|e| e.to_string())?;
        let num_tags = u16::from_le_bytes(count_buf);

        for _ in 0..num_tags {
            let tag = read_tiff_tag(&mut rawfile);
            
            match tag.id {
                0x0110 => {
                    let model = read_tiff_to_string(&mut rawfile, tag.offset, tag.count);
                    meta.model = model;
                    println!("Model: {}", &meta.model);
                },
                0x010F => {
                    let make = read_tiff_to_string(&mut rawfile, tag.offset, tag.count);
                    meta.make = make;
                    println!("Make: {}", meta.make);
                },
                0x8769 => {
                    println!("Found EXIF Pointer! Room is at: {}", tag.offset);
                    meta.exif = parse_exif(&mut rawfile, meta.make.clone(), tag.offset).ok();
                },
                _ => {} // Ignore others for now
            }
        }
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
    return  Ok(meta)
}

fn parse_exif(rawfile: &mut File, make: String, exif_offset: u32) -> Result<ExifData, String> {
    let saved_pos = rawfile.stream_position().unwrap();

    rawfile.seek(SeekFrom::Start(exif_offset as u64)).map_err(|e| e.to_string())?;
        // Read the 2-byte count for THIS room u16::from_le_bytes(exif_count_buf);

    // let mut exif_count_buf = [0u8; 2];
    // rawfile.read_exact(&mut exif_count_buf).unwrap();
    let num_tags = read_u16(rawfile);
    let mut data = ExifData { iso: 0, exposure_time: 0, makernote_offset: 0, wb_gains: (1.0, 1.0, 1.0) };
    println!("--- EXIF Room has {} tags ---", num_tags);
    for _ in 0..num_tags {
        let exif_tag = read_tiff_tag(rawfile);

        match exif_tag.id {
            0x8827 => data.iso = exif_tag.offset,
            0x829A => data.exposure_time = exif_tag.offset, 
            0x927C => {
                    data.makernote_offset = exif_tag.offset;
                    println!("Found MakerNote Pointer: {}", exif_tag.offset);
                    let real_makernote_addr = exif_offset as u64;
                    let make_lower = make.to_lowercase();
                    if make_lower.contains("canon") {
                        // Pass the reference directly. No need for data = *parse...
                        data = parse_canon_makernote(rawfile, &mut data, exif_tag.offset);
                        println!("Testing Absolute: {}", exif_tag.offset);
                        parse_canon_makernote(rawfile, &mut data, exif_tag.offset);
    
                        // TEST 2: If Test 1 didn't find "Canon", try the offset + 12
                        // (Sometimes the pointer points to the Tag entry, not the data)
                        if data.wb_gains.0 == 1.0 {
                             println!("Trying Offset + 12: {}", exif_tag.offset + 12);
                             parse_canon_makernote(rawfile, &mut data, exif_tag.offset + 12);
                        }
                    } else if make_lower.contains("nikon") {
                        // parse_nikon_makernote(rawfile, &mut data, exif_tag.offset);
                    } else {
                        println!("Make '{}' did not trigger Canon/Nikon logic", make_lower);
                    }
                },
            _ => {}
        }
    }
    rawfile.seek(SeekFrom::Start(saved_pos)).ok(); // Jump back!
    println!("iso {:?} exposure {:?}  makenote {:?}", &data.iso, &data.exposure_time, &data.makernote_offset);
    Ok(data)
}

fn parse_canon_makernote(rawfile: &mut File, exif_data: &mut ExifData, offset: u32) ->  ExifData {
    let saved_pos = rawfile.stream_position().unwrap();
    let makernote_start = offset as u64;

    let mut probe = [0u8; 16];
    rawfile.read_exact(&mut probe).unwrap();
    
    // This print is the most important part of your current journey:
    println!("🔍 BYTES AT {}: {:02X?}", makernote_start, probe);

    rawfile.seek(SeekFrom::Start(makernote_start)).unwrap();

    let mut magic = [0u8; 8];
    if rawfile.read_exact(&mut magic).is_err() {
        return *exif_data;
    }

    let actual_ifd_start: u64;
    if &magic[0..5] == b"Canon" {
        println!("reached 5ds block");
        // High-end (5DS/5DmkIV): The real IFD starts 8 bytes in
        actual_ifd_start = makernote_start + 8;
    } else {
        // Entry-level (1200D/600D): Starts immediately
        actual_ifd_start = makernote_start;
    }

    rawfile.seek(SeekFrom::Start(actual_ifd_start)).unwrap();
    let num_tags = read_u16(rawfile);

    let file_len = rawfile.metadata().unwrap().len();
    let remaining_bytes = file_len.saturating_sub(actual_ifd_start);
    let max_possible_tags = (remaining_bytes / 12) as u16;

    let file_len = rawfile.metadata().unwrap().len();

    for _ in 0..num_tags {
        let tag = read_tiff_tag(rawfile);
        
        // Canon Rule: Internal offsets are relative to the MakerNote START
        let target_offset = makernote_start + tag.offset as u64;

        // Safety check to prevent panics
        if target_offset >= file_len { break; }

        match tag.id {
            0x0001 => {
                println!("🎯 color block found at relative offset: {}", tag.offset);
                exif_data.wb_gains = extract_canon_wb(rawfile, target_offset);
            }
            _ => {println!("doing something")} // Silence unknowns for now
        }
    }
    rawfile.seek(SeekFrom::Start(saved_pos)).ok(); 
    *exif_data
}


//helper functions

fn read_u16(file: &mut File) -> u16 {
    let mut buf = [0u8; 2];
    file.read_exact(&mut buf).unwrap();
    u16::from_le_bytes(buf)
}

fn read_tiff_tag(file: &mut File) -> TiffTag {
    let mut buf = [0u8; 12];
    file.read_exact(&mut buf).unwrap();
    TiffTag {
        id: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
        data_type: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
        count: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
        offset: u32::from_le_bytes(buf[8..12].try_into().unwrap())
    }
}

fn read_tiff_to_string(file: &mut File, offset: u32, count:u32) -> String {
    let current_pos = file.stream_position().unwrap();

    file.seek(SeekFrom::Start(offset as u64)).unwrap();

    let mut buf = vec![0u8; count as usize];
    file.read_exact(&mut buf).unwrap();

    file.seek(SeekFrom::Start(current_pos)).unwrap();
    String::from_utf8_lossy(&buf).trim_matches(|c: char| c == '\0' || c.is_whitespace())
        .to_string()
}

fn extract_canon_wb(rawfile: &mut File, abs_offset: u64) -> (f32, f32, f32) {
    let saved_pos = rawfile.stream_position().unwrap();
    rawfile.seek(SeekFrom::Start(abs_offset)).unwrap();
    
    // 0u8 is an empty buffer. 0x8 was a buffer full of the number 8!
    let mut buf = [0u8; 80]; // Read enough to reach the high-end table
    if rawfile.read_exact(&mut buf).is_err() { return (1.0, 1.0, 1.0); }

    // High-end (5DS, 5D Mk III, etc.) usually uses index 72
    // Entry-level (1200D, 600D) uses index 2
    let mut wb_r = u16::from_le_bytes(buf[72..74].try_into().unwrap()) as f32;
    let mut wb_g = u16::from_le_bytes(buf[74..76].try_into().unwrap()) as f32;
    let mut wb_b = u16::from_le_bytes(buf[76..78].try_into().unwrap()) as f32;

    // Fallback if the high-end index is empty/zero
    if wb_g == 0.0 {
        wb_r = u16::from_le_bytes(buf[2..4].try_into().unwrap()) as f32;
        wb_g = u16::from_le_bytes(buf[4..6].try_into().unwrap()) as f32;
        wb_b = u16::from_le_bytes(buf[6..8].try_into().unwrap()) as f32;
    }
    
    println!("RAW Gains -> R: {}, G: {}, B: {}", wb_r, wb_g, wb_b);
    rawfile.seek(SeekFrom::Start(saved_pos)).ok();

    if wb_g > 0.0 {
        let r_gain = wb_r / wb_g;
        let b_gain = wb_b / wb_g;
        println!("Normalized Gains -> Red: {:.3}, Blue: {:.3}", r_gain, b_gain);
        (r_gain, 1.0, b_gain)
    } else {
        (1.0, 1.0, 1.0)
    }
}