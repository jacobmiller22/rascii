// use std;

use clap::Parser;
use image::{
    imageops::FilterType, io::Reader as ImageReader, DynamicImage, GenericImageView, Pixel, Pixels,
    Rgba,
};

// fn read_image(filename: &str) -> DynamicImage {}
const ASCII_CHARS: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`\'. ";

// The ratio between the width and height of a single character
const CHAR_RATIO: (u32, u32) = (3, 5); // The width of a character is half its height

fn main() {
    let args: CliArgs = CliArgs::parse();

    println!("path: {:?}", args.path);

    let img: DynamicImage = ImageReader::open(args.path).unwrap().decode().unwrap();

    // Resize the image to a width of 100px
    let img = img.resize(100, 100, FilterType::Nearest);

    paint(img);
}

#[derive(Parser)]
pub struct CliArgs {
    /// Path to file
    pub path: std::path::PathBuf,
    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // pub count: u8,
}

fn paint(img: DynamicImage) {
    let (width, height) = img.dimensions();
    let pixels: Pixels<DynamicImage> = img.pixels();

    let mut ascii_img = String::new();

    for (x, y, pixel) in pixels {
        // // Skip rows and columns to conform to the character ratio
        // if (CHAR_RATIO.0 != 1 && y % CHAR_RATIO.0 == 0)
        //     || (CHAR_RATIO.1 != 1 && y % CHAR_RATIO.1 == 0)
        // {
        //     continue;
        // }

        let mut v = vec![pixel]; // The pixels that will be averaged

        // // If the next row or column will be skipped,consider it in the average
        // if (CHAR_RATIO.0 != 1 && y % CHAR_RATIO.0 == CHAR_RATIO.0 - 1)
        //     || (CHAR_RATIO.1 != 1 && y % CHAR_RATIO.1 == CHAR_RATIO.1 - 1)
        // {
        //     // Get the pixels in the next row or column
        //     if CHAR_RATIO.0 != 1 && y % CHAR_RATIO.0 == CHAR_RATIO.0 - 1 && y != height - 1 {
        //         v.push(img.get_pixel(x, y + 1));
        //     }
        //     if CHAR_RATIO.1 != 1 && y % CHAR_RATIO.1 == CHAR_RATIO.1 - 1 && x != width - 1 {
        //         v.push(img.get_pixel(x + 1, y));
        //     }
        // }

        // Get the character that corresponds to the pixel
        let c: char = pixels_to_char(v);

        ascii_img.push(c);

        // Check if we are at the end of a row
        if x == width - 1 {
            ascii_img.push('\n');
        }
    }

    print!("{}", ascii_img);
}

fn pixels_to_char(pixels: Vec<Rgba<u8>>) -> char {
    // Average the pixel values
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;

    let n: u32 = pixels.len() as u32;

    for pixel in pixels {
        let (pr, pg, pb, _) = pixel.channels4();
        r += pr as u32;
        g += pg as u32;
        b += pb as u32;
    }

    let avg: u32 = (r + g + b) / (3 * n);

    // Find the character that corresponds to the average pixel value
    let index = (avg as f32 * ((ASCII_CHARS.len() - 1) as f32 / 255.0)) as usize;
    // println!(
    //     "{}, {}, {}",
    //     avg,
    //     index,
    //     ASCII_CHARS.chars().nth(index).unwrap()
    // );

    ASCII_CHARS.chars().nth(index).unwrap()
}
