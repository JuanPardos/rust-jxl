use jpegxl_rs::encode::EncoderSpeed;
use image::io::Reader as ImageReader;
use std::fs::{create_dir_all, write};
use jpegxl_rs::encoder_builder;
use std::path::{Path, PathBuf};
use std::io::stdin;
use glob::glob;

const FILE_MASK: [&str; 4] = ["png", "jpg", "jpeg", "webp"];
const DEFAULT_QUALITY: f32 = 2.0;
const DEFAULT_EFFORT: u8 = 7;

fn main() {
    let options = main_menu();
    let images = retrieve_images(&options[0]);
    let mut exit = String::new();
    create_output_folder();
    for (index, i) in images.iter().enumerate() {
        println!("Compressing image: {} of {}", index + 1, images.len());
        compress_image(&i, options[1].parse::<f32>().expect("Invalid quality value"), options[2].parse::<u8>().expect("Invalid effort value"));
    }
    println!("Process completed. Check the 'output' folder.");
    println!("Press Enter to exit...");
    stdin().read_line(&mut exit).unwrap();
}

fn main_menu() -> Vec<String> {
    let mut path = String::new();
    let mut quality = String::new();
    let mut effort = String::new();

    println!("Welcome to the JPEG XL Compression Tool!");

    println!("Please enter the path of the image folder: ");
    stdin().read_line(&mut path).expect("Failed to read input");

    println!("Please enter the desired quality (0.0-15.0). Default: {DEFAULT_QUALITY}");
    stdin().read_line(&mut quality).expect("Failed to read input");

    println!("Please enter the desired effort (1-9). Default: {DEFAULT_EFFORT}");
    stdin().read_line(&mut effort).expect("Failed to read input");

    path = path.trim().to_string();
    let quality = if quality.trim().is_empty() { DEFAULT_QUALITY.to_string() } else { quality.trim().to_string() };
    let effort = if effort.trim().is_empty() { DEFAULT_EFFORT.to_string() } else { effort.trim().to_string() };

    vec![path, quality, effort]
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
    println!("{} images found.", images_path.len());
    images_path
}

fn create_output_folder() {
    let output_path = Path::new("output");
    create_dir_all(&output_path).expect("Failed to create output directory");
}

fn compress_image(image_path: &Path, quality: f32, speed: u8) {
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    let rgb = img.to_rgb16();
    let (width, height) = rgb.dimensions();
    let pixels: Vec<u16> = rgb.pixels().flat_map(|p| p.0.iter().copied()).collect();

    let encoder_speed = match speed {
        1 => EncoderSpeed::Lightning,
        2 => EncoderSpeed::Thunder,
        3 => EncoderSpeed::Falcon,
        4 => EncoderSpeed::Cheetah,
        5 => EncoderSpeed::Hare,
        6 => EncoderSpeed::Wombat,
        7 => EncoderSpeed::Squirrel,
        8 => EncoderSpeed::Kitten,
        9 => EncoderSpeed::Tortoise,
        _ => EncoderSpeed::Squirrel,
    };

    let mut encoder = encoder_builder()
        .quality(quality)
        .speed(encoder_speed)
        .lossless(false)
        .build()
        .expect("Failed to build encoder");

    let encoded = encoder.encode::<u16, u16>(&pixels, width, height)
        .expect("Error encoding image");
    let buffer = encoded.as_ref();

    let file_stem = image_path.file_stem().unwrap().to_string_lossy();
    let output_path = Path::new("output").join(format!("{}.jxl", file_stem));
    write(&output_path, buffer).expect("Failed to write JXL file");
}
