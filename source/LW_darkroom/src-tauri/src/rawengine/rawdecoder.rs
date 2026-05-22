use std::path::{Path, PathBuf};
use image::{ImageBuffer, Rgb};
use rawloader::decode_file;
use rawloader::RawImageData;
use std::cmp::max;

pub fn extract_thumbnails(path: &Path, cache_path: &Path) -> Option<PathBuf> {
    let img = decode_file(path).ok()?;

    let width = img.width as usize;
    let height = img.height as usize;
    let pixel_count = width * height;
    let data = match img.data {
        RawImageData::Integer(data) => data,
        _ => return None,
    };
    let max_value = *data.iter().max()? as f32;

    // let mut rgb = Vec::with_capacity((width * height * 3) as usize);
    let rgb = rgb_guess(&data,width,height,max_value);

    

        let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(
            width as u32,
            height as u32,
            rgb, // Vec<u8> RGB buffer expected
    )?;

    let thumb = image::imageops::resize(
        &buffer,
        256,
        256,
        image::imageops::FilterType::Triangle,
    );


    thumb.save(cache_path).ok()?;

    Some(cache_path.to_path_buf())
}
pub fn get_cache_path(path: &Path) -> PathBuf {
    let file_name = path.file_stem().unwrap().to_string_lossy();

    let mut cache = std::env::temp_dir();
    cache.push("tauri_thumbs");

    let _ = std::fs::create_dir_all(&cache);

    cache.push(format!("{file_name}.jpg"));
    cache
}

pub fn rgb_guess(data: &[u16], width: usize, height: usize, max_value: f32) -> Vec<u8> {
    let mut rgb = vec![0u8; width * height * 3];
    
    // Canon CR2 Specifics
    let black_level = 1024.0; 
    let saturation_level = max_value; // Usually around 13000-15000 for CR2
    let range = saturation_level - black_level;

    // Standard Canon daylight WB gains (Approximate)
    let r_gain = 2.0;
    let g_gain = 1.0;
    let b_gain = 1.5;

    for y in 0..height as isize {
        for x in 0..width as isize {
            let i = (y as usize * width + x as usize) * 3;
            let val = get(data, x, y, width as isize, height as isize);

            // 1. Identify channel based on RGGB pattern
            let (mut r, mut g, mut b) = match (x % 2 == 0, y % 2 == 0) {
                (true, true) => (val, 0.0, 0.0),   // Red pixel
                (false, false) => (0.0, 0.0, val), // Blue pixel
                _ => (0.0, val, 0.0),             // Green pixels
            };

            // Simple Interpolation (Bilinear)
            if r > 0.0 {
                g = (get(data, x-1, y, width as isize, height as isize) + get(data, x+1, y, width as isize, height as isize)) * 0.5;
                b = (get(data, x-1, y-1, width as isize, height as isize) + get(data, x+1, y+1, width as isize, height as isize)) * 0.5;
            } else if b > 0.0 {
                g = (get(data, x-1, y, width as isize, height as isize) + get(data, x+1, y, width as isize, height as isize)) * 0.5;
                r = (get(data, x-1, y-1, width as isize, height as isize) + get(data, x+1, y+1, width as isize, height as isize)) * 0.5;
            } else {
                r = (get(data, x-1, y, width as isize, height as isize) + get(data, x+1, y, width as isize, height as isize)) * 0.5;
                b = (get(data, x, y-1, width as isize, height as isize) + get(data, x, y+1, width as isize, height as isize)) * 0.5;
            }

            // Processing Pipeline: Subtract Black -> Gain -> Normalize -> Gamma
            let mut process = |v: f32, gain: f32| {
                let v = ((v - black_level) / range).max(0.0) * gain;
                // Apply Gamma 2.2
                (v.powf(1.0 / 2.2) * 255.0).clamp(0.0, 255.0) as u8
            };

            rgb[i]     = process(r, r_gain);
            rgb[i + 1] = process(g, g_gain);
            rgb[i + 2] = process(b, b_gain);
        }
    }
    rgb
}

fn get(data: &[u16], x: isize, y: isize, w: isize, h: isize) -> f32 {
    if x < 0 || y < 0 || x >= w || y >= h {
        return 0.0;
    }
    data[(y as usize * w as usize + x as usize)] as f32
}