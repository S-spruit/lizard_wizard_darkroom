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

    // // for pixel in data {
    // //     let value = (pixel >> 8) as u8;

    // //     rgb.push(value);
    // //     rgb.push(value);
    // //     rgb.push(value);
    // // }
    // let mut r_sum: f32 = 0.0;
    // let mut g_sum: f32 = 0.0;
    // let mut b_sum: f32 = 0.0;
    
    // let mut r_count: f32 = 0.0;
    // let mut g_count: f32 = 0.0;
    // let mut b_count: f32 = 0.0;

    // for i in 0..(width * height) as usize {
    //     if i >= data.len() {
    //         break;
    //     }

    //     let x = i % width;
    //     let y = i / width;

    //     let v = data[i] as f32;

    //     // 👉 PURE per-pixel Bayer reconstruction
    //     let (r, g, b) = match (x % 2, y % 2) {

    //         // RGGB (common Canon CR2 assumption)
    //         (0, 0) => (v, 0.6 * v, 0.0),
    //         (1, 0) => (0.0, v, 0.0),
    //         (0, 1) => (0.0, v, 0.0),
    //         (1, 1) => (0.0, 0.6 * v, v),

    //         _ => (v, v, v),
    //     };

    //     let idx = i * 3;

    //     rgb[idx]     = ((r / max_value) * 255.0).clamp(0.0, 255.0) as u8;
    //     rgb[idx + 1] = ((g / max_value) * 255.0).clamp(0.0, 255.0) as u8;
    //     rgb[idx + 2] = ((b / max_value) * 255.0).clamp(0.0, 255.0) as u8;
    // }

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
    for y in 0..height as isize {
    for x in 0..width as isize {

        let i = (y as usize * width + x as usize) * 3;

        let c = data[(y as usize * width + x as usize)] as f32;

        let is_red =  x % 2 == 0 && y % 2 == 0;
        let is_green1 = x % 2 == 1 && y % 2 == 0;
        let is_green2 = x % 2 == 0 && y % 2 == 1;
        let is_blue = x % 2 == 1 && y % 2 == 1;

        let r: f32;
        let g: f32;
        let b: f32;

        // if is_green {
        //     // green pixel → interpolate R and B
        //     g = c;
        //     r = (get(&data, x - 1, y, width as isize, height as isize)
        //        + get(&data, x + 1, y, width as isize, height as isize)) * 0.5;

        //     b = (get(&data, x, y - 1, width as isize, height as isize)
        //        + get(&data, x, y + 1, width as isize, height as isize)) * 0.5;
        // } else if (x % 2 == 0) {
        //     // red pixel
        //     r = c;
        //     g = (
        //         get(&data, x - 1, y, width as isize, height as isize) +
        //         get(&data, x + 1, y, width as isize, height as isize) +
        //         get(&data, x, y - 1, width as isize, height as isize) +
        //         get(&data, x, y + 1, width as isize, height as isize)
        //     ) * 0.25;

        //     b = (
        //         get(&data, x - 1, y - 1, width as isize, height as isize) +
        //         get(&data, x + 1, y - 1, width as isize, height as isize) +
        //         get(&data, x - 1, y + 1, width as isize, height as isize) +
        //         get(&data, x + 1, y + 1, width as isize, height as isize)
        //     ) * 0.25;
        // } else {
        //     // blue pixel
        //     b = c;
        //     g = (
        //         get(&data, x - 1, y, width as isize, height as isize) +
        //         get(&data, x + 1, y, width as isize, height as isize) +
        //         get(&data, x, y - 1, width as isize, height as isize) +
        //         get(&data, x, y + 1, width as isize, height as isize)
        //     ) * 0.25;

        //     r = (
        //         get(&data, x - 1, y - 1, width as isize, height as isize) +
        //         get(&data, x + 1, y - 1, width as isize, height as isize) +
        //         get(&data, x - 1, y + 1, width as isize, height as isize) +
        //         get(&data, x + 1, y + 1, width as isize, height as isize)
        //     ) * 0.25;
        // }
        let (r, g, b) = if is_red {
            let r = c;
            let g = (get(data, x - 1, y, width as isize, height as isize) + get(data, x + 1, y, width as isize, height as isize)) * 0.5;
            let b = (get(data, x, y - 1, width as isize, height as isize) + get(data, x, y + 1, width as isize, height as isize)) * 0.5;
            (r, g, b)

        } else if is_blue {
            let b = c;
            let g = (get(data, x - 1, y, width as isize, height as isize) + get(data, x + 1, y, width as isize, height as isize)) * 0.5;
            let r = (get(data, x, y - 1, width as isize, height as isize) + get(data, x, y + 1, width as isize, height as isize)) * 0.5;
            (r, g, b)
        
        } else {
            // green pixels (both types)
            let g = c;
            let r = (get(data, x - 1, y, width as isize, height as isize) + get(data, x + 1, y, width as isize, height as isize)) * 0.5;
            let b = (get(data, x, y - 1, width as isize, height as isize) + get(data, x, y + 1, width as isize, height as isize)) * 0.5;
            (r, g, b)
        };
        let r_gain = 1.8;
        let g_gain = 1.0;
        let b_gain = 1.4;

        rgb[i]     = ((r * r_gain) / max_value * 255.0).clamp(0.0, 255.0) as u8;
        rgb[i + 1] = ((g * g_gain) / max_value * 255.0).clamp(0.0, 255.0) as u8;
        rgb[i + 2] = ((b * b_gain) / max_value * 255.0).clamp(0.0, 255.0) as u8;
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