pub mod parser;
pub mod effects;
pub mod utils;

use parser::Args;
use clap::Parser;


fn main() {
    let args = Args::parse();
    args.validate();

    let res = image::open(args.src_path);

    if let Err(err) = res {
        println!("Unable to load image: {}", err.to_string());
        return;
    }

    let mut image = res.unwrap();

    if args.effect.to_lowercase() == "grayscale" {
        effects::grayscale(&mut image);
    } else if args.effect.to_lowercase() == "edgedetect" {
        effects::edge_detect(&mut image);
    } else if args.effect.to_lowercase() == "invertcolor" {
        effects::invert_color(&mut image);
    } else if args.effect.to_lowercase() == "floodfill" {
        //effects::flood_fill(&mut image, 0, 0, image::Rgba([0, 0, 0, 0]));
        effects::flood_fill(&mut image, 0, 0, image::Rgba([255, 105, 180, 0]));
    }

    image.save_with_format(args.dst_path, image::ImageFormat::Png).unwrap();
}