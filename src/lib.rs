use image::{io::Reader as ImageReader, GenericImageView, Rgba, Pixel, DynamicImage};

pub fn read_image(filename: &str) -> DynamicImage {
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
      // print!("{}", ascii_pixel.character);
  }
}

pub fn pixel_as_char(pixel: Rgba<u8>) -> u8 {

  let ascii_chars: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`\'. ";
  let (r, g, b, a) = pixel.channels4();
  let mut sum: u32 = 0;
  sum += r as u32;
  sum += g as u32;
  sum += b as u32; 

  
  let avg: u8 = (sum / 3) as u8;
  let index = (avg as f32 / 3.7 as f32) as usize;
  ascii_chars.chars().nth(ascii_chars.len() - index - 1).unwrap() as u8
}