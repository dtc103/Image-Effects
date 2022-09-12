pub mod image_functions {

    pub mod misc {
        pub fn open_picture() -> image::RgbImage {
            println!("Pfad des Bildes eingeben: ");
            let mut path = String::new();
            std::io::stdin()
                .read_line(&mut path)
                .expect("Path Reading went wrong");
            path = String::from(path.trim());

            let images = image::open(path).expect("something went wrong");

            images.into_rgb()
        }

        pub fn hsv_to_rgb(mut h: f32, mut s: f32, mut v: f32) -> (u8, u8, u8) {
            while h >= 360f32 {
                h -= 360f32;
            }
            if h < 0f32 {
                h = 0f32;
            }
        
            if s < 0f32 {
                s = 0f32;
            }
            if s > 1f32 {
                s = 1f32;
            }
        
            if v < 0f32 {
                v = 0f32;
            }
            if v > 1f32 {
                v = 1f32;
            }
        
            let c = v * s;
            let x = c * (1f32 - ((h / 60f32) % 2f32 - 1f32).abs());
            let m = v - c;
        
            //h has to be in the interval [0, 360) (because of the first while loop in this function)
            // the conversion to u32 also leads to the fact, that we cant use all the values on the circle, but only 360
            match h as u32 {
                0..=59 => (
                    ((c + m) * 255f32) as u8,
                    ((x + m) * 255f32) as u8,
                    (m * 255f32) as u8,
                ),
                60..=119 => (
                    ((x + m) * 255f32) as u8,
                    ((c + m) * 255f32) as u8,
                    (m * 255f32) as u8,
                ),
                120..=179 => (
                    (m * 255f32) as u8,
                    ((c + m) * 255f32) as u8,
                    ((x + m) * 255f32) as u8,
                ),
                180..=239 => (
                    (m * 255f32) as u8,
                    ((x + m) * 255f32) as u8,
                    ((c + m) * 255f32) as u8,
                ),
                240..=299 => (
                    ((x + m) * 255f32) as u8,
                    (m * 255f32) as u8,
                    ((c + m) * 255f32) as u8,
                ),
                300..=359 => (
                    ((c + m) * 255f32) as u8,
                    (m * 255f32) as u8,
                    ((x + m) * 255f32) as u8,
                ),
                _ => panic!("This case shouldnt be happen"),
            }
        }
    }

    pub mod picture_generation{
        use rand;
        use rand::prelude::*;
        use rand::seq::SliceRandom;
        use super::misc::hsv_to_rgb;

        pub fn chaos_game(
            picture_size: (u32, u32),
            iteration_depth: u32,
            input_points: Option<Vec<(u32, u32)>>,
            colored: bool,
            mut relative_jump: f32, //should be bigger than 1
        ) -> Option<image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>> {
            let mut image = image::RgbImage::new(picture_size.0, picture_size.1);
        
            if relative_jump <= 1f32 {
                relative_jump = 1f32;
            }
        
            let steps = iteration_depth;
        
            //only use these 3 points, if the user doesnt provide own points
            let a: (u32, u32);
            let b: (u32, u32);
            let c: (u32, u32);
            let mut point_array = Vec::new();
            match input_points {
                Some(mut input_list) => {
                    for point in &input_list {
                        //wont check for < 0, because the type checker will throw an exception if that happens.
                        if point.0 > image.width() || point.1 > image.height() {
                            return None;
                        }
                    }
                    point_array.append(&mut input_list);
                }
                None => {
                    a = (image.width() / 2, (image.height() as f32 * 0.02) as u32);
                    b = (
                        (image.width() as f32 * 0.02) as u32,
                        (image.height() as f32 * 0.98) as u32,
                    );
                    c = (
                        (image.width() as f32 * 0.98) as u32,
                        (image.height() as f32 * 0.98) as u32,
                    );
                    point_array.append(&mut vec![a, b, c]);
                }
            };
            //first point is in the middle of the picture(Could be at random position, but this doesnt rly matter).
            let mut point = (image.width() / 2, image.height() / 2);
        
            let mut rnd = thread_rng();
        
            for _ in 0..steps {
                let corner = point_array.as_slice().choose(&mut rnd).unwrap();
                point = (
                    (point.0 as i32 + ((corner.0 as i32 - point.0 as i32) as f32 / relative_jump) as i32)
                        as u32,
                    (point.1 as i32 + ((corner.1 as i32 - point.1 as i32) as f32 / relative_jump) as i32)
                        as u32,
                );
        
                if colored {
                    let color: (u8, u8, u8) = hsv_to_rgb(
                        360f32 * ((image.height() - point.1) as f32 / image.height() as f32),
                        1f32,
                        1f32,
                    );
                    image.put_pixel(point.0, point.1, image::Rgb([color.0, color.1, color.2]));
                } else {
                    image.put_pixel(point.0, point.1, image::Rgb([255, 255, 255]));
                }
            }
        
            Some(image)
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
}
