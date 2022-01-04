use std;
use image::{io::Reader as ImageReader, GenericImageView, Rgba, Pixel};

fn parse_args() ->Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    args
}

fn find_character_for_color(rgba: Rgba<u8>) -> u8 {
    
    // print!("{}", Rgba(rgba).data[0]);
    let ascii_chars: Vec<u8> = vec!['$' as u8, '@' as u8, 'B' as u8, '%' as u8, '8' as u8, '&' as u8, 'W' as u8, 'M' as u8, '#' as u8, '*' as u8, 'o' as u8, 'a' as u8, 'h' as u8, 'k' as u8, 'b' as u8, 'd' as u8, 'p' as u8, 'q' as u8, 'w' as u8, 'm' as u8, 'Z' as u8, 'O' as u8, '0' as u8, 'Q' as u8, 'L' as u8, 'C' as u8, 'J' as u8, 'U' as u8, 'Y' as u8, 'X' as u8, 'z' as u8, 'c' as u8, 'v' as u8, 'u' as u8, 'n' as u8, 'x' as u8, 'r' as u8, 'j' as u8, 'f' as u8, 't' as u8, '/' as u8, '\\' as u8, '|' as u8, '(' as u8, ')' as u8, '1' as u8, '{' as u8, '}' as u8, '[' as u8, ']' as u8, '?' as u8, '-' as u8, '_' as u8, '+' as u8, '~' as u8, '<' as u8, '>' as u8, 'i' as u8, '!' as u8, 'l' as u8, 'I' as u8, ';' as u8, ':' as u8, ',' as u8, '"' as u8, '^' as u8, '`' as u8, '\'' as u8, '.' as u8, ' ' as u8];

    
    let (r, g, b, a) = rgba.channels4();
    
    let mut sum: u32 = 0;
    sum += r as u32;
    sum += g as u32;
    sum += b as u32; 

    let avg: u8 = (sum / 12) as u8;

    // print!("{}-", avg);
    
    ascii_chars[ avg as usize]
}


fn main() {
    let args = parse_args();
    
    let filename = &args[1];
    
    

    // Decode the file
    
    let img = ImageReader::open(filename).unwrap().decode().unwrap();


    let gray_image = img.grayscale();

    let pixels = gray_image.pixels();


    struct AsciiPixel {
        x: u32,
        y: u32,
        character: u8,
    }


    let mut ascii_pixels: Vec<AsciiPixel> = vec![];

    for pixel in pixels {
        let (x, y, rgba) = pixel;
        // let avg = (r + g + b) / 3; // Average of the RGB values
        
        // Need to find an ascii character whose average darkness is closest to the average of the RGB values
        ascii_pixels.push(AsciiPixel {
            x,
            y,
            character: find_character_for_color(rgba),
        });
    }

    let mut _y: u32 = 0;

    let mut row: Vec<u8> = vec![];
    for ascii_pixel in ascii_pixels {
        if ascii_pixel.y > _y {
            // &mut row.push('\n' as u8);
            _y = ascii_pixel.y;
            println!("{}", String::from_utf8(row).unwrap());
            row = vec![]; // Vec consumed, so we need to make a new one
        }
        row.push(ascii_pixel.character);
        // print!("{}", ascii_pixel.character);
    }
    println!("Image: {}\nWidth: {}\nHeight: {}", filename, img.width(), img.height());

    

}
