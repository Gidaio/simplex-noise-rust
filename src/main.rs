#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};


fn main() {
    let mut image = Image::new(256, 256);

    for (x, y) in image.coordinates() {
        image.set_pixel(x, y, px!(x, y, 0))
    }

    image.save("image.bmp").unwrap();
}
