pub mod image_functions {

    pub mod misc {
        fn open_picture() -> image::RgbImage {
            println!("Pfad des Bildes eingeben: ");
            let mut path = String::new();
            std::io::stdin()
                .read_line(&mut path)
                .expect("Path Reading went wrong");
            path = String::from(path.trim());

            let images = image::open(path).expect("something went wrong");

            images.into_rgb()
        }
    }
    pub mod filter {
        pub fn grey_filter(
            img: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
        ) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
            let mut new_pic = image::ImageBuffer::new(img.width(), img.height());
            for y in 0..img.height() {
                for x in 0..img.width() {
                    let curr_pixel = img.get_pixel(x, y);
                    let grey_value = (curr_pixel[0] as f32 * 0.31) as u8
                        + (curr_pixel[1] as f32 * 0.59) as u8
                        + (curr_pixel[2] as f32 * 0.11) as u8;
                    let new_pixel = image::Rgb([grey_value, grey_value, grey_value]);
                    new_pic.put_pixel(x, y, new_pixel);
                }
            }
            new_pic
        }
        pub fn filter3x3(
            img: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
            kernel: &[[f32; 3]; 3],
        ) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
            let mut new_pic: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> =
                image::ImageBuffer::new(img.width(), img.height());
            let (height, width) = (new_pic.height(), new_pic.width());

            for y in 0..height {
                for x in 0..width {
                    let mut r = 0f32;
                    let mut g = 0f32;
                    let mut b = 0f32;
                    for ker_y in 0..kernel.len() as i32 {
                        for ker_x in 0..kernel[0].len() as i32 {
                            if x as i32 + ker_x - 1 < 0 || x as i32 + ker_x - 1 >= width as i32 {
                                continue;
                            }
                            if y as i32 + ker_y - 1 < 0 || y as i32 + ker_y - 1 >= height as i32 {
                                continue;
                            }

                            r += kernel[ker_y as usize][ker_x as usize]
                                * img.get_pixel(x + ker_x as u32 - 1, y + ker_y as u32 - 1)[0]
                                    as f32;
                            g += kernel[ker_y as usize][ker_x as usize]
                                * img.get_pixel(x + ker_x as u32 - 1, y + ker_y as u32 - 1)[1]
                                    as f32;
                            b += kernel[ker_y as usize][ker_x as usize]
                                * img.get_pixel(x + ker_x as u32 - 1, y + ker_y as u32 - 1)[2]
                                    as f32;
                        }
                    }
                    if r < 0.0 {
                        r = 0.0;
                    }
                    if r > 255.0 {
                        r = 255.0
                    }
                    if g < 0.0 {
                        g = 0.0;
                    }
                    if g > 255.0 {
                        g = 255.0;
                    }
                    if b < 0.0 {
                        b = 0.0;
                    }
                    if b > 255.0 {
                        b = 255.0;
                    }
                    let new_pixel = image::Rgb([r as u8, g as u8, b as u8]);
                    new_pic.put_pixel(x, y, new_pixel);
                }
            }

            new_pic
        }

        pub fn blur(
            img: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
        ) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
            image::ImageBuffer::new(1, 1)
        }
    }

    pub mod fractals {
        use rand;
        use rand::prelude::*;
        use rand::seq::SliceRandom;

        pub fn chaos_game(
            picture_size: (u32, u32),
            iteration_depth: u32,
        ) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
            let mut image = image::RgbImage::new(picture_size.0, picture_size.1);

            let steps = iteration_depth;

            let a = (image.width() / 2, (image.height() as f32 * 0.02) as u32);
            let b = (
                (image.width() as f32 * 0.02) as u32,
                (image.height() as f32 * 0.98) as u32,
            );
            let c = (
                (image.width() as f32 * 0.98) as u32,
                (image.height() as f32 * 0.98) as u32,
            );

            let point_array = vec![a, b, c];
            let mut point = (image.width() / 2, image.width() / 2);

            let mut rnd = thread_rng();

            for _ in 0..steps {
                let corner = point_array.as_slice().choose(&mut rnd).unwrap();
                point = (
                    (point.0 as i32 + ((corner.0 as i32 - point.0 as i32) / 2)) as u32,
                    (point.1 as i32 + ((corner.1 as i32 - point.1 as i32) / 2)) as u32,
                );

                image.put_pixel(point.0, point.1, image::Rgb([255, 255, 255]));
            }

            image
        }
    }
}
