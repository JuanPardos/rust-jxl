use jpegxl_rs::parallel::threads_runner::ThreadsRunner;
use image::{GenericImageView, ImageReader};
use std::fs::{create_dir_all, write};
use jpegxl_rs::encode::EncoderSpeed;
use std::io::{stdin, stdout, Write};
use jpegxl_rs::encoder_builder;
use std::path::{Path, PathBuf};
use image::ColorType;
use glob::glob;

const FILE_MASK: [&str; 4] = ["png", "jpg", "jpeg", "webp"];
const DEFAULT_QUALITY: f32 = 1.5;
const DEFAULT_EFFORT: u8 = 6;

//TODO: Better error handling. Input validation.
fn main() {
    let options = main_menu();
    let images = retrieve_images(&options[0]);
    let mut exit = String::new();
    create_output_folder();
    for (index, i) in images.iter().enumerate() {
        println!("Compressing image: {} of {}", index + 1, images.len());
        compress_image(&i, options[1].parse::<f32>().expect("Invalid quality value"), options[2].parse::<u8>().expect("Invalid effort value"));
    }
    println!("\nProcess completed. Check the 'output' folder.");
    stdin().read_line(&mut exit).unwrap();
}

fn main_menu() -> Vec<String> {
    let mut path = String::new();
    let mut quality = String::new();
    let mut effort = String::new();

    println!("===============================================");
    println!("=        JPEG XL Image Compression Tool       =");
    println!("===============================================");
    println!("This program allows you to compress images to JPEG XL (.jxl) format.");
    println!("Supported formats: PNG, JPG, JPEG, WEBP.");
    println!("You can adjust compression quality and effort (speed).");
    println!("-----------------------------------------------");

    println!("1. Enter the folder path containing images:");
    println!("   Example: /home/juan/pictures/");
    print!("   > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut path).expect("Failed to read input");

    println!("\n2. Enter desired quality (0.0-15.0)");
    println!("   Lower value = higher quality and larger files. Recommended: 0.5-4.0 || Default: {DEFAULT_QUALITY}");
    print!("   > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut quality).expect("Failed to read input");

    println!("\n3. Enter compression effort (1-10)");
    println!("   Lower value = faster but worse compression. Recommended: 3-9 || Default: {DEFAULT_EFFORT}");
    print!("   > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut effort).expect("Failed to read input");

    path = path.trim().to_string();
    let quality = if quality.trim().is_empty() { DEFAULT_QUALITY.to_string() } else { quality.trim().to_string() };
    let effort = if effort.trim().is_empty() { DEFAULT_EFFORT.to_string() } else { effort.trim().to_string() };

    println!("\nStarting compression...\n");

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
    println!("{} images found. \n", images_path.len());
    images_path
}

fn create_output_folder() {
    let output_path = Path::new("output");
    create_dir_all(&output_path).expect("Failed to create output directory");
}

fn compress_image(image_path: &Path, quality: f32, speed: u8) {
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .with_guessed_format()
        .expect("Failed to guess format")
        .decode()
        .expect("Failed to decode image");

    let (width, height) = img.dimensions();

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
        10 => EncoderSpeed::Glacier,
        _ => EncoderSpeed::Wombat,
    };

    let threads_runner = ThreadsRunner::default();

    //TODO: Lossless compression
    let mut encoder = encoder_builder()
        .quality(quality)
        .speed(encoder_speed)
        .lossless(false)
        .parallel_runner(&threads_runner)
        .build()
        .expect("Failed to build encoder");

    let file_stem = image_path.file_stem().unwrap().to_string_lossy();
    let output_path = Path::new("output").join(format!("{}.jxl", file_stem));

    let buffer = match img.color() {
        //TODO: Alpha channel support (RGBA, transparency)
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
            println!("Unsupported color type for image: {:?}", img.color());
            return;
        }
    };

    write(&output_path, &buffer).expect("Failed to write JXL file");
}
