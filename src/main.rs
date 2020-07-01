use image::*;

mod picture_manipulation;
use picture_manipulation::image_functions::filter::filter3x3;
use picture_manipulation::image_functions::filter::grey_filter;
use picture_manipulation::image_functions::fractals::chaos_game;

use std::io;

fn main() {
    //let rgb_image = open_picture();
    let image = chaos_game((512, 512), 100000000);

    image
        .save_with_format("fractal.png", ImageFormat::Png)
        .expect("Fractal was generated, but saving failed");
}

fn open_picture() -> RgbImage {
    println!("Pfad des Bildes eingeben: ");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Path Reading went wrong");
    path = String::from(path.trim());

    let images = image::open(path).expect("something went wrong");

    images.into_rgb()
}
