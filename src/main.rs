use image::*;

mod picture_manipulation;
use picture_manipulation::image_functions::misc::open_picture;
use picture_manipulation::image_functions::filter::filter3x3;
use picture_manipulation::image_functions::filter::grey_filter;

//use picture_manipulation::image_functions::fractals::chaos_game;

use std::io;

fn main() {
    let rgb_image = open_picture();

    rgb_image
        .save_with_format("fractal.png", ImageFormat::Png)
        .expect("Fractal was generated, but saving failed");
}