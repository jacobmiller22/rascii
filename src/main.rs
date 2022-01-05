use std;
mod lib;
use crate::lib::{read_image, paint_image_as_ascii};

fn parse_args() ->Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    args
}


fn main() {
    let args = parse_args();
    
    let filename = &args[1];
    
    let img = read_image(filename);
    paint_image_as_ascii(img);

}
