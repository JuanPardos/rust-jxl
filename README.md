**rust-jxl** is a simple command-line utility written in Rust to convert and compress images into the high-efficiency JPEG XL format.

Also check out the AVIF tool: https://github.com/JuanPardos/rust-image-compress

More info about JPEG XL: https://jpegxl.info/

## Features

- Consider this an improved version of the AVIF tool (they say JPEG XL is faster and better ☝🤓)
- Standalone program, libjxl is statically linked so there is no need to install it
- Fully offline, no need to be ashamed of your anime wallpapers
- Lossy compression: typically reduces file size to 20% (or less) of the original, with no noticeable loss in visual quality
- Lossless mode available
- Supports multiple input formats: PNG, JPG, JPEG, and WebP
- Configurable compression via quality and effort (speed). Tuned by default for a good balance of speed, quality and file size
- Batch processing of all images in a directory
- Rustacean speed with multi-thread support
- Tested on Linux and Windows

## TODO

- Better error handling
- Alpha channel support (transparency)

## Requirements

- Rust toolchain
- C++ build tools
- cmake
- clang
- *Additional build dependencies may be required. 

## Building

1. Clone the repository:
   ```bash
   git clone https://github.com/JuanPardos/rust-jxl
   cd rust-jxl
   ```

2. Build in release mode:
   ```bash
   cargo build --release
   ```

3. The executable will be located at:
   ```bash
   target/release/rust-jxl
   ```

4. (Optional) Compress the executable with UPX:
   ```bash
   upx -9 rust-jxl
   ```

### Compressed binaries for Linux and Windows are also available in the Releases section

## Usage

Run the program and follow instructions:

```bash
./rust-jxl
```

You will be prompted for:

1. **Path to the images folder**: directory containing your images.
2. **Do you want to use lossless mode?**: using lossless mode gives the best results (if you don't mind about file size), disables any choices.  
Default N.
3. **Quality (0.0-15.0)**: sets the quality. Lower values = Higher quality and larger files.  
Recommended 0.5-4.0  
Default 1.5.
4. **Effort (1-10)**: determines the compression effort. Lower values = Faster but worse compression.  
Recommended 3-9  
Default: 6.

## Notes

- This tool never overwrites your original images, all compressed files are saved in a separate `output` folder.  
- JPEG XL format is not widely supported. Depending on your operating system, you may need to install additional libraries or extensions to use JPEG XL. You may encounter issues, I recommend using this as a storage tool. However, you should be able to recover the original format using other tools.
- Support for transparency might be broken, you will probably lose it and get a black background instead.

