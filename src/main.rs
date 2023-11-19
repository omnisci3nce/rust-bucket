use image::{Rgb, RgbImage};
use nalgebra::Point2;
/*
Full rendering pipeline:
1. clear buffers
2.
*/

// pub struct Triangle

type Point = Point2<i32>;

fn line(a: Point, b: Point, image_buffer: &mut RgbImage, color: Rgb<u8>) {
    let mut a_x = a.x;
    let mut a_y = a.y;
    let mut b_x = b.x;
    let mut b_y = b.y;
    let mut steep = false;

    if (a_x - b_x).abs() < (a_y - b_y).abs() {
        // line is steep so we transpose the image
        steep = true;
        std::mem::swap(&mut a_x, &mut a_y);
        std::mem::swap(&mut b_x, &mut b_y);
    }
    if a_x > b_x {
        // always have it left-to-right (b.x > a.x)
        std::mem::swap(&mut a_x, &mut b_x);
        std::mem::swap(&mut a_y, &mut b_y);
    }
    let dx = b_x - a_x;
    let dy = b_y - a_y;
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = a_y;
    for x in a_x..=b_x {
        if steep {
            image_buffer.put_pixel(y as u32, x as u32, color); // if transposed, de-transpose
        } else {
            image_buffer.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > dx {
            if b_y > a_y {
                y += 1
            } else {
                y -= 1
            };
            error -= dx * 2;
        }
    }
}

fn main() {
    let mut img = RgbImage::new(128, 128);

    for x in 0..128 {
        for y in 0..128 {
            img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }

    // White horizontal-ish line
    line(
        Point2::new(10, 90),
        Point2::new(100, 70),
        &mut img,
        image::Rgb([255, 255, 255]),
    );
    // Red vertical-ish line
    line(
        Point2::new(20, 100),
        Point2::new(60, 10),
        &mut img,
        image::Rgb([255, 0, 0]),
    );

    if let Err(e) = img.save("image.png") {
        eprintln!("Error saving image: {e}");
    }
}
