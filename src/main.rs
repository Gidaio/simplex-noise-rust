extern crate bmp;
use bmp::{Image, Pixel};
use rand::prelude::*;


fn main() {
    let mut image = Image::new(256, 256);

    for (x, y) in image.coordinates() {
        image.set_pixel(x, y, Pixel::new(random(), random(), random()))
    }

    image.save("image.bmp").unwrap();
}
