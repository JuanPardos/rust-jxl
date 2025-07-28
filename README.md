**rust-jxl** is a command-line utility written in Rust for bidirectional conversion between JPEG XL and other image formats.

Also check out the AVIF tool: https://github.com/JuanPardos/rust-image-compress

More info about JPEG XL: https://jpegxl.info/

## Features

- Bidirectional conversion:
  - **Compression**: Convert images (PNG, JPG, JPEG, WebP) to high-efficiency JPEG XL format
  - **Decompression**: Convert JPEG XL images back to PNG format with optimization
- Consider this an improved version of the AVIF tool (they say JPEG XL is faster and better ☝🤓)
- Standalone program, libjxl is statically linked so there is no need to install it
- Fully offline, no need to be ashamed of your anime wallpapers
- **Compression features:**
  - Lossy compression: typically reduces file size to 20% (or less) of the original, with no noticeable loss in visual quality
  - Lossless mode available
  - Configurable compression via quality and effort (speed). Tuned by default for a good balance of speed, quality and file size
- **Decompression features:**
  - Converts JXL images back to PNG format
  - Automatic PNG optimization using Oxipng for smaller file sizes
  - Supports all JXL color formats (RGB, RGBA, Grayscale) with automatic conversion to RGB PNG
  - Warning about potential file size increase (JXL is more efficient than PNG)
- Supports multiple input formats: PNG, JPG, JPEG, WebP (for compression), JXL (for decompression)
- Status report with compression ratio and progress
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

### Conversion Modes

The program supports two conversion modes:

#### 1. Compress images to JXL (default)
You will be prompted for:
1. **Path to the images folder**: directory containing your images (PNG, JPG, JPEG, WebP).
2. **Do you want to use lossy compression?**: default is lossy compression, select lossless for maximum quality (and size).  
   Default Y (Lossy).
3. **Effort (1-10)**: determines the compression effort. Lower values = Faster but worse compression.  
   Recommended 3-8  
   Default: 6.
4. **Quality (0.0-15.0)**: sets the quality. Lower values = Higher quality and larger files. Disabled if lossless. 
   Recommended 0.5-4.0  
   Default 1.5.

#### 2. Convert JXL to PNG
You will be prompted for:
1. **Path to the JXL folder**: directory containing your .jxl files.

The program will:
- Decode JXL files to PNG format
- Automatically optimize the PNG output using Oxipng for better compression
- Show a warning about potential file size increase (JXL is more efficient than PNG)
- Support all JXL color formats with automatic conversion to standard RGB PNG

## Notes

- This tool never overwrites your original images, all processed files are saved in a separate `output` folder.  
- **For JXL compression**: JPEG XL format is not widely supported. Depending on your operating system, you may need to install additional libraries or extensions to use JPEG XL. You may encounter issues, I recommend using this as a storage tool. However, you should be able to recover the original format using the JXL to PNG conversion feature or other tools.
- **For JXL to PNG conversion**: Converting from JXL to PNG may result in larger file sizes as JXL is a more efficient format. The tool uses Oxipng to optimize the PNG output, but file sizes will typically be larger than the original JXL.
- Support for transparency might be limited during JXL compression - you will probably lose it and get a black background instead. The JXL to PNG conversion handles transparency by converting RGBA to RGB.

