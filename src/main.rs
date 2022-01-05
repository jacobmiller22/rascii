use std;
mod lib;
use crate::lib::{read_image, paint_image_as_ascii, print_usage, AspectRatio};
#[derive(Default)]
struct Args {
    file_path: String,
    samplings: String,
    aspect_ratio: AspectRatio,
    palette: String,
}

fn parse_args() ->Args {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(0);
    }   

    let mut flag = None;

    let mut arguments = Args::default();   

    for raw_arg in args.iter() {
        // Check flags first
        if flag == Some("-S") {
            flag = None;
            // This is value of -S flag
            arguments.samplings = raw_arg.to_string();
            continue;
        } 
        else if flag == Some("-A") {
            flag = None;
            // This is value of -A flag
            let aspect_ratio_vec = raw_arg.split(":").collect::<Vec<&str>>();
            if aspect_ratio_vec.len() != 2 {
                println!("Invalid aspect ratio: {}", raw_arg);
                std::process::exit(1);
            }
            
            arguments.aspect_ratio.width = Some(aspect_ratio_vec[0].parse::<u32>().unwrap());
            arguments.aspect_ratio.height = Some(aspect_ratio_vec[1].parse::<u32>().unwrap());
            continue;
        } 
        else if flag == Some("-P") {
            flag = None;
            // This is value of -P flag
            arguments.file_path = raw_arg.to_string();
            continue;
        }

        // Then check arguments for flags or arguments
        let arg = raw_arg.to_lowercase();
        if arg == "-h" || arg == "--help" {
            print_usage(&args[0]);
            std::process::exit(0);
        }
        else if arg == "-s" || arg == "--sampling" {
            flag = Some("-S");
        }
        else if arg == "-a" || arg == "--aspect-ratio" {
            flag = Some("-A");
        }
        else if arg == "-p" || arg == "--palette" {
            flag = Some("-P");
        } 
        else {
            // Must be our file path
            arguments.file_path = raw_arg.to_string();
        }
    }
    arguments
}


fn main() {
    let args = parse_args();
    
    let file_path = args.file_path;
    
    let img = read_image(&file_path);
    paint_image_as_ascii(img);

}
