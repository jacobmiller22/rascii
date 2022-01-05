use image::{io::Reader as ImageReader, GenericImageView, Rgba, Pixel, DynamicImage};

#[derive(Default)]
pub struct AspectRatio {
  pub width: Option<u32>,
  pub height: Option<u32>,
}

pub fn read_image(filename: &String) -> DynamicImage {
  let img = ImageReader::open(filename).unwrap().decode().unwrap();
  img
}

pub fn paint_image_as_ascii(img: DynamicImage) -> () {
  let pixels = img.pixels();

  struct AsciiPixel {
      x: u32,
      y: u32,
      character: u8,
  }

  let mut ascii_pixels: Vec<AsciiPixel> = vec![];

  for pix_desc in pixels {
      let (x, y, rgba) = pix_desc;
      // let avg = (r + g + b) / 3; // Average of the RGB values
      
      // Need to find an ascii character whose average darkness is closest to the average of the RGB values
      ascii_pixels.push(AsciiPixel {
          x,
          y,
          character: pixel_as_char(rgba),
      });
  }

  let mut _y: u32 = 0;

  let mut row: Vec<u8> = vec![];
  for ascii_pixel in ascii_pixels {
      if ascii_pixel.y % 2 == 0 {
          continue;
      }
      if ascii_pixel.y > _y {
          // &mut row.push('\n' as u8);
          _y = ascii_pixel.y;
          println!("{}", String::from_utf8(row).unwrap());
          row = vec![]; // Vec consumed, so we need to make a new one
      }
      row.push(ascii_pixel.character);
  }
}

pub fn pixel_as_char(pixel: Rgba<u8>) -> u8 {

  let ascii_chars: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`\'. ";
  let (r, g, b, _) = pixel.channels4();
  let mut sum: u32 = 0;
  sum += r as u32;
  sum += g as u32;
  sum += b as u32; 

  
  let avg: u8 = (sum / 3) as u8;
  let index = (avg as f32 / 3.7 as f32) as usize;
  ascii_chars.chars().nth(ascii_chars.len() - index - 1).unwrap() as u8

}

pub fn print_usage(program_name: &String) -> () {
    print!(
    "Usage: {} [OPTIONS]... [FILE]...\n
    Prints an ascii image representation of the image corresponding to filename FILE to standard output.\n
    Mandatory arguments to long options are mandatory for short options too.\n
    \t-H, --help\t\tdisplay this message\n
    \t-S, --sampling\t\tchoose sampling type. OPTIONS: <NONE>, <SKIP>, <BLEND>\n
    \t-A, --aspect-ratio\tDefine the aspect ratio of the text to assist in BLEND sampling. Use format <WIDTH>:<HEIGHT>\n
    \t-P, --palette\t\tProvide a custom palette to use for pixel mappings. Characters in palette should be ordered from highest to lowest intensity\n", program_name);  
}