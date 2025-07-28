use jpegxl_rs::parallel::threads_runner::ThreadsRunner;
use image::{GenericImageView, ImageReader, RgbImage, DynamicImage};
use std::fs::{create_dir_all, write};
use jpegxl_rs::encode::EncoderSpeed;
use std::io::{stdin, stdout, Write};
use jpegxl_rs::{encoder_builder, decoder_builder};
use std::path::{Path, PathBuf};
use image::ColorType;
use glob::glob;
use std::fs;
use oxipng::{optimize, Options, OutFile, InFile};

const FILE_MASK: [&str; 4] = ["png", "jpg", "jpeg", "webp"];
const JXL_FILE_MASK: [&str; 1] = ["jxl"];
const DEFAULT_QUALITY: f32 = 1.5;
const DEFAULT_EFFORT: u8 = 6;

//TODO: Better error handling. Input validation.
fn main() {
    let options = main_menu();
    let conversion_mode = &options[0]; // "compress" or "decode"
    let images = retrieve_images(&options[1], conversion_mode);
    let mut exit = String::new();
    create_output_folder();
    let mut skipped = false;

    if conversion_mode == "compress" {
        let lossy = options[4].parse::<bool>().unwrap_or(true);

        for (index, i) in images.iter().enumerate() {
            let file_name = i.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let original_size = fs::metadata(i).map(|m| m.len()).unwrap_or(0);

            if lossy {
                skipped = compress_image(&i, options[2].parse::<u8>().expect("Invalid effort value"), options[3].parse::<f32>().expect("Invalid quality value"), lossy);
            } else {
                skipped = compress_image(&i, options[2].parse::<u8>().expect("Invalid effort value"), 0.0, lossy);
            }

            let file_stem = i.file_stem().unwrap().to_string_lossy();
            let output_path = Path::new("output").join(format!("{}.jxl", file_stem));
            let final_size = fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0);

            let percent = if original_size > 0 {
                (final_size as f64 / original_size as f64) * 100.0
            } else {
                0.0
            };

            if skipped {
                println!("[{}/{}] SKIPPED {}", index + 1, images.len(), file_name);
            } else {
                println!("[{}/{}] ({:.2}%) {}", index + 1, images.len(), percent, file_name);
            }
        }

        if skipped {
            println!("\nWARNING: Some images were not processed because the compressed file was larger than the original or other issues occurred. These were copied to the output folder.");
        } 
    } else if conversion_mode == "decode" {
        println!("\nWARNING: Converting from JXL to PNG may result in larger file sizes. JXL is a more efficient format than PNG.");
        
        for (index, i) in images.iter().enumerate() {
            let file_name = i.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let original_size = fs::metadata(i).map(|m| m.len()).unwrap_or(0);

            skipped = decode_jxl_to_png(&i);

            let file_stem = i.file_stem().unwrap().to_string_lossy();
            let output_path = Path::new("output").join(format!("{}.png", file_stem));
            let final_size = fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0);

            let percent = if original_size > 0 {
                (final_size as f64 / original_size as f64) * 100.0
            } else {
                0.0
            };

            if skipped {
                println!("[{}/{}] SKIPPED {}", index + 1, images.len(), file_name);
            } else {
                println!("[{}/{}] ({:.2}%) {}", index + 1, images.len(), percent, file_name);
            }
        }

        if skipped {
            println!("\nWARNING: Some JXL images could not be processed due to errors. These were skipped.");
        }
    }

    println!("\nProcess completed. Check the 'output' folder.");
    stdin().read_line(&mut exit).unwrap();
}

fn main_menu() -> Vec<String> {
    let mut input_path = String::new();
    let mut input_quality = String::new();
    let mut input_effort = String::new();
    let mut input_lossy = String::new();
    let mut input_mode = String::new();
    let mut options = Vec::new();

    let mut quality_val = String::new();

    println!("===============================================");
    println!("=        JPEG XL Image Conversion Tool        =");
    println!("===============================================");
    println!("This program allows you to:");
    println!("1. Compress images to JPEG XL (.jxl) format");
    println!("2. Convert JPEG XL images back to PNG format");
    println!("Supported input formats: PNG, JPG, JPEG, WEBP (for compression), JXL (for conversion to PNG).\n");

    println!("Choose conversion mode:");
    println!("1. Compress images to JXL [c/compress] (default)");
    println!("2. Convert JXL to PNG [d/decode]");
    print!("   > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_mode).expect("Failed to read input");
    let mode = input_mode.trim().to_lowercase();
    let conversion_mode = match mode.as_str() {
        "d" | "decode" | "2" => "decode",
        _ => "compress",
    };

    options.push(conversion_mode.to_string());

    println!("\nEnter the folder path containing images:");
    if conversion_mode == "compress" {
        println!(r##"   Example: /home/juan/pictures/  or  C:\Users\juan\Pictures\"##);
        println!("   (Looking for PNG, JPG, JPEG, WEBP files)");
    } else {
        println!(r##"   Example: /home/juan/jxl_images/  or  C:\Users\juan\JXL_Images\"##);
        println!("   (Looking for JXL files)");
    }
    print!("   > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_path).expect("Failed to read input");
    options.push(input_path.trim().to_string());

    if conversion_mode == "compress" {
        println!("\nDo you want to use lossy compression? [y=Lossy(default), n=Lossless]");
        print!("   > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input_lossy).expect("Failed to read input");
        let use_lossy = input_lossy.trim().is_empty() || input_lossy.trim().to_lowercase() == "y";

        println!("\nEnter compression effort (1-10)");
        println!("   Lower value = faster but worse compression. Recommended: 3-8 || Default: {DEFAULT_EFFORT}");
        print!("   > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input_effort).expect("Failed to read input");
        let effort_val = if input_effort.trim().is_empty() { DEFAULT_EFFORT.to_string() } else { input_effort.trim().to_string() };

        if use_lossy {
            println!("\nEnter desired quality (0.0-15.0)");
            println!("   Lower value = higher quality and larger files. Recommended: 0.5-4.0 || Default: {DEFAULT_QUALITY}");
            print!("   > ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input_quality).expect("Failed to read input");
            quality_val = if input_quality.trim().is_empty() { DEFAULT_QUALITY.to_string() } else { input_quality.trim().to_string() };
        }

        options.push(effort_val);
        options.push(quality_val);
        options.push(use_lossy.to_string());
    } else {
        // For decode mode, we don't need compression settings
        options.push("0".to_string()); // effort placeholder
        options.push("0.0".to_string()); // quality placeholder
        options.push("false".to_string()); // lossy placeholder
    }

    println!("\nStarting {}...\n", if conversion_mode == "compress" { "compression" } else { "conversion to PNG" });
    options
}

fn retrieve_images(path: &str, mode: &str) -> Vec<PathBuf> {
    let mut images_path = Vec::new();
    let clean_path = Path::new(path);

    if mode == "decode" {
        for ext in JXL_FILE_MASK {
            let pattern = clean_path.join(format!("*.{}", ext));
            let pattern_str = pattern.to_string_lossy();
            for entry in glob(&pattern_str).expect("Error reading pattern") {
                match entry {
                    Ok(path) => images_path.push(path),
                    Err(e) => eprintln!("Error reading files: {}", e),
                }
            }
        }
    } else {
        for ext in FILE_MASK {
            let pattern = clean_path.join(format!("*.{}", ext));
            let pattern_str = pattern.to_string_lossy();
            for entry in glob(&pattern_str).expect("Error reading pattern") {
                match entry {
                    Ok(path) => images_path.push(path),
                    Err(e) => eprintln!("Error reading files: {}", e),
                }
            }
        }
    }

    println!("{} {} files found. \n", images_path.len(), if mode == "decode" { "JXL" } else { "image" });
    images_path
}

fn create_output_folder() {
    let output_path = Path::new("output");
    create_dir_all(&output_path).expect("Failed to create output directory");
}

fn compress_image(image_path: &Path, effort: u8, quality: f32, lossy: bool) -> bool {
    let mut skipped = false;
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .with_guessed_format()
        .expect("Failed to guess format")
        .decode()
        .expect("Failed to decode image");

    let (width, height) = img.dimensions();

    let encoder_speed = match effort {
        1 => EncoderSpeed::Lightning,
        2 => EncoderSpeed::Thunder,
        3 => EncoderSpeed::Falcon,
        4 => EncoderSpeed::Cheetah,
        5 => EncoderSpeed::Hare,
        6 => EncoderSpeed::Wombat,
        7 => EncoderSpeed::Squirrel,
        8 => EncoderSpeed::Kitten,
        9 => EncoderSpeed::Tortoise,
        10 => EncoderSpeed::Glacier,
        _ => EncoderSpeed::Wombat,
    };

    let threads_runner = ThreadsRunner::default();
    let mut base_builder = encoder_builder();
    let builder = base_builder
        .speed(encoder_speed)
        .quality(quality)
        .lossless(!lossy)
        .parallel_runner(&threads_runner);

    if !lossy {
        builder.uses_original_profile(true);
    }

    let mut encoder = builder.build().expect("Failed to build encoder");

    let file_stem = image_path.file_stem().unwrap().to_string_lossy();
    let output_path = Path::new("output").join(format!("{}.jxl", file_stem));

    let buffer = match img.color() {
        ColorType::Rgb8 | ColorType::Rgba8 => {
            let rgb = img.to_rgb8();
            let pixels: Vec<u8> = rgb.pixels().flat_map(|p| p.0.iter().copied()).collect();
            let encoded = encoder.encode::<u8, u8>(&pixels, width, height)
                .expect("Error encoding image");
            encoded.as_ref().to_vec()
        }
        ColorType::Rgb16 | ColorType::Rgba16 => {
            let rgb = img.to_rgb16();
            let pixels: Vec<u16> = rgb.pixels().flat_map(|p| p.0.iter().copied()).collect();
            let encoded = encoder.encode::<u16, u16>(&pixels, width, height)
                .expect("Error encoding image");
            encoded.as_ref().to_vec()
        }
        _ => {
            let ext = image_path.extension().and_then(|e| e.to_str()).unwrap_or("img");
            let file_stem = image_path.file_stem().unwrap().to_string_lossy();
            let original_output = Path::new("output").join(format!("{}.{}", file_stem, ext));
            fs::copy(image_path, &original_output).expect("Failed to copy file");
            return true; 
        }
    };

    let original_size = fs::metadata(image_path).map(|m| m.len()).unwrap_or(0);
    let compressed_size = buffer.len() as u64;

    if compressed_size >= original_size {
        skipped = true;
        let ext = image_path.extension().and_then(|e| e.to_str()).unwrap_or("img");
        let original_output = Path::new("output").join(format!("{}.{}", file_stem, ext));
        fs::copy(image_path, &original_output).expect("Failed to copy file");
    } else {
        write(&output_path, &buffer).expect("Failed to write JXL file");
    }
    skipped
}

fn decode_jxl_to_png(jxl_path: &Path) -> bool {
    let skipped = false;
    
    // Read the JXL file
    let jxl_data = match fs::read(jxl_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read JXL file {}: {}", jxl_path.display(), e);
            return true;
        }
    };

    // Decode JXL
    let threads_runner = ThreadsRunner::default();
    let decoder = decoder_builder()
        .parallel_runner(&threads_runner)
        .build()
        .expect("Failed to build decoder");

    let (info, decoded_data) = match decoder.decode(&jxl_data) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to decode JXL file {}: {}", jxl_path.display(), e);
            return true;
        }
    };

    // Extract pixel data from the Pixels enum
    let pixel_data: Vec<u8> = match decoded_data {
        jpegxl_rs::decode::Pixels::Uint8(data) => data,
        jpegxl_rs::decode::Pixels::Uint16(data) => {
            // Convert u16 to u8 by dividing by 256
            data.iter().map(|&x| (x / 256) as u8).collect()
        }
        jpegxl_rs::decode::Pixels::Float(data) => {
            // Convert f32 to u8
            data.iter().map(|&x| (x * 255.0).clamp(0.0, 255.0) as u8).collect()
        }
        jpegxl_rs::decode::Pixels::Float16(data) => {
            // Convert f16 to u8
            data.iter().map(|&x| (x.to_f32() * 255.0).clamp(0.0, 255.0) as u8).collect()
        }
    };

    // Create image from decoded data
    let img = match info.num_color_channels {
        3 => {
            // RGB
            match RgbImage::from_raw(info.width, info.height, pixel_data) {
                Some(rgb_img) => DynamicImage::ImageRgb8(rgb_img),
                None => {
                    eprintln!("Failed to create RGB image from decoded data for {}", jxl_path.display());
                    return true;
                }
            }
        }
        4 => {
            // RGBA - convert to RGB for PNG
            let rgba_data: Vec<u8> = pixel_data.chunks_exact(4)
                .flat_map(|rgba| &rgba[0..3])
                .copied()
                .collect();
            match RgbImage::from_raw(info.width, info.height, rgba_data) {
                Some(rgb_img) => DynamicImage::ImageRgb8(rgb_img),
                None => {
                    eprintln!("Failed to create RGB image from RGBA data for {}", jxl_path.display());
                    return true;
                }
            }
        }
        1 => {
            // Grayscale - convert to RGB
            let rgb_data: Vec<u8> = pixel_data.iter()
                .flat_map(|&gray| [gray, gray, gray])
                .collect();
            match RgbImage::from_raw(info.width, info.height, rgb_data) {
                Some(rgb_img) => DynamicImage::ImageRgb8(rgb_img),
                None => {
                    eprintln!("Failed to create RGB image from grayscale data for {}", jxl_path.display());
                    return true;
                }
            }
        }
        _ => {
            eprintln!("Unsupported number of color channels {} for {}", info.num_color_channels, jxl_path.display());
            return true;
        }
    };

    // Save as PNG first
    let file_stem = jxl_path.file_stem().unwrap().to_string_lossy();
    let temp_png_path = Path::new("output").join(format!("{}_temp.png", file_stem));
    let final_png_path = Path::new("output").join(format!("{}.png", file_stem));

    if let Err(e) = img.save(&temp_png_path) {
        eprintln!("Failed to save PNG file {}: {}", temp_png_path.display(), e);
        return true;
    }

    // Optimize with oxipng
    let options = Options::from_preset(6); // High compression preset
    let out_file = OutFile::Path { 
        path: Some(final_png_path.clone()), 
        preserve_attrs: false 
    };
    match optimize(&InFile::Path(temp_png_path.clone()), &out_file, &options) {
        Ok(_) => {
            // Remove temp file
            let _ = fs::remove_file(&temp_png_path);
        }
        Err(e) => {
            eprintln!("Failed to optimize PNG with oxipng for {}: {}", temp_png_path.display(), e);
            // If optimization fails, just rename the temp file
            if let Err(rename_err) = fs::rename(&temp_png_path, &final_png_path) {
                eprintln!("Failed to rename temp PNG file: {}", rename_err);
                return true;
            }
        }
    }

    skipped
}
