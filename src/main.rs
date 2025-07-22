use jpegxl_rs::parallel::threads_runner::ThreadsRunner;
use image::{GenericImageView, ImageReader};
use std::fs::{create_dir_all, write};
use jpegxl_rs::encode::EncoderSpeed;
use std::io::{stdin, stdout, Write};
use jpegxl_rs::encoder_builder;
use std::path::{Path, PathBuf};
use image::ColorType;
use glob::glob;
use std::fs;

const FILE_MASK: [&str; 4] = ["png", "jpg", "jpeg", "webp"];
const DEFAULT_QUALITY: f32 = 1.5;
const DEFAULT_EFFORT: u8 = 6;

//TODO: Better error handling. Input validation.
fn main() {
    let options = main_menu();
    let images = retrieve_images(&options[0]);
    let mut exit = String::new();
    create_output_folder();
    let lossy = options[3].parse::<bool>().unwrap_or(true);
    let mut skipped = false;

    for (index, i) in images.iter().enumerate() {
        let file_name = i.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let original_size = fs::metadata(i).map(|m| m.len()).unwrap_or(0);

        if lossy {
            skipped = compress_image(&i, options[1].parse::<u8>().expect("Invalid effort value"), options[2].parse::<f32>().expect("Invalid quality value"), lossy);
        } else {
            skipped = compress_image(&i, options[1].parse::<u8>().expect("Invalid effort value"), 0.0, lossy);
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

    println!("\nProcess completed. Check the 'output' folder.");
    stdin().read_line(&mut exit).unwrap();
}

fn main_menu() -> Vec<String> {
    let mut input_path = String::new();
    let mut input_quality = String::new();
    let mut input_effort = String::new();
    let mut input_lossy = String::new();
    let mut options = Vec::new();

    let mut quality_val = String::new();

    println!("===============================================");
    println!("=        JPEG XL Image Compression Tool       =");
    println!("===============================================");
    println!("This program allows you to compress images to JPEG XL (.jxl) format.");
    println!("Supported formats: PNG, JPG, JPEG, WEBP.");
    println!("You can configure it yourself or use lossless mode.\n");

    println!("1. Enter the folder path containing images:");
    println!(r##"   Example: /home/juan/pictures/  or  C:\Users\juan\Pictures\"##);
    print!("   > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_path).expect("Failed to read input");

    println!("\n2. Do you want to use lossy compression? [y=Lossy(default), n=Lossless]");
    print!("   > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_lossy).expect("Failed to read input");
    let use_lossy = input_lossy.trim().is_empty() || input_lossy.trim().to_lowercase() == "y";

    println!("\n3. Enter compression effort (1-10)");
    println!("   Lower value = faster but worse compression. Recommended: 3-8 || Default: {DEFAULT_EFFORT}");
    print!("   > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_effort).expect("Failed to read input");
    let effort_val = if input_effort.trim().is_empty() { DEFAULT_EFFORT.to_string() } else { input_effort.trim().to_string() };

    if use_lossy {
        println!("\n4. Enter desired quality (0.0-15.0)");
        println!("   Lower value = higher quality and larger files. Recommended: 0.5-4.0 || Default: {DEFAULT_QUALITY}");
        print!("   > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input_quality).expect("Failed to read input");
        quality_val = if input_quality.trim().is_empty() { DEFAULT_QUALITY.to_string() } else { input_quality.trim().to_string() };
    }

    options.push(input_path.trim().to_string());
    options.push(effort_val);
    options.push(quality_val);
    options.push(use_lossy.to_string());

    println!("\nStarting compression...\n");
    options
}

fn retrieve_images(path: &str) -> Vec<PathBuf> {
    let mut images_path = Vec::new();
    let clean_path = Path::new(path);

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

    println!("{} images found. \n", images_path.len());
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
