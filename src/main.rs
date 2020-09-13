use bmp::{Image, Pixel};
// use rand::prelude::*;


static IMAGE_WIDTH: u32 = 256;
static IMAGE_HEIGHT: u32 = 256;
static GRID_WIDTH: i32 = 16;
static GRID_HEIGHT: i32 = 16;


#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, b: Vector) -> Vector {
        Vector { x: self.x - b.x, y: self.y - b.y }
    }
}

impl std::ops::Mul for Vector {
    type Output = f64;

    fn mul(self, b: Vector) -> f64 {
        self.x * b.x + self.y * b.y
    }
}


fn main() {
    let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut min = 0f64;
    let mut max = 0f64;

    for (pixel_x, pixel_y) in image.coordinates() {
        println!("Pixel ({}, {})", pixel_x, pixel_y);
        let grid_position = pixel_to_grid(pixel_x, pixel_y);
        println!("Grid ({}, {})", grid_position.x, grid_position.y);
        let value = simplex_noise(grid_position);
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
        let grey = noise_to_greyscale(value);
        image.set_pixel(pixel_x, pixel_y, Pixel::new(grey, grey, grey));
        println!("");
    }

    println!("Range: {} - {}", min, max);

    image.save("image.bmp").unwrap();
}


fn pixel_to_grid(pixel_x: u32, pixel_y: u32) -> Vector {
    let grid_x = pixel_x as f64 / IMAGE_WIDTH as f64 * GRID_WIDTH as f64;
    let grid_y = pixel_y as f64 / IMAGE_HEIGHT as f64 * GRID_HEIGHT as f64;

    Vector { x: grid_x, y: grid_y }
}


fn simplex_noise(grid_position: Vector) -> f64 {
    let skewed_position = skew(grid_position);
    println!("Skewed ({}, {})", skewed_position.x, skewed_position.y);
    let simplex_positions = get_simplex_positions(skewed_position);
    let gradients = get_gradients(simplex_positions);

    let mut sum: f64 = 0.0;

    for i in 0..3 {
        println!("Simplex {} ({}, {})", i, simplex_positions[i].x, simplex_positions[i].y);
        println!("Gradient {} ({}, {})", i, gradients[i].x, gradients[i].y);
        let unskewed_simplex_vertex = unskew(simplex_positions[i]);
        println!("Unskewed simplex {} ({}, {})", i, unskewed_simplex_vertex.x, unskewed_simplex_vertex.y);
        let distance = grid_position - unskewed_simplex_vertex;
        println!("Distance {} ({}, {})", i, distance.x, distance.y);
        let gradient = distance * gradients[i];
        println!("Gradient {}: {}", i, gradient);
        let contribution = 0.6 * 0.6 - distance * distance;
        println!("Contribution {}: {}", i, contribution);

        sum += if contribution <= 0.0 {
            0f64
        } else {
            contribution * contribution * contribution * contribution * gradient
        }
    }

    println!("Sum: {}", sum);

    sum * 476f64
}


fn skew(grid_position: Vector) -> Vector {
    let factor = (grid_position.x + grid_position.y) * (3f64.sqrt() - 1.0) / 2.0;

    Vector { x: grid_position.x + factor, y: grid_position.y + factor }
}


fn unskew(skewed_position: Vector) -> Vector {
    let factor = (skewed_position.x + skewed_position.y) * (1f64 - 1f64 / 3f64.sqrt()) / 2f64;

    Vector { x: skewed_position.x - factor, y: skewed_position.y - factor }
}


fn get_simplex_positions(skewed_position: Vector) -> [Vector; 3] {
    if skewed_position.x >= skewed_position.y {
        [
            Vector { x: skewed_position.x.floor(), y: skewed_position.y.floor() },
            Vector { x: skewed_position.x.ceil(), y: skewed_position.y.floor() },
            Vector { x: skewed_position.x.ceil(), y: skewed_position.y.ceil() },
        ]
    } else {
        [
            Vector { x: skewed_position.x.floor(), y: skewed_position.y.floor() },
            Vector { x: skewed_position.x.floor(), y: skewed_position.y.ceil() },
            Vector { x: skewed_position.x.ceil(), y: skewed_position.y.ceil() },
        ]
    }
}


fn get_gradients(simplex_positions: [Vector; 3]) -> [Vector; 3] {
    let mut out = [Vector { x: 0.0, y: 0.0 }, Vector { x: 0.0, y: 0.0 }, Vector { x: 0.0, y: 0.0 }];

    for i in 0..3 {
        let direction = (simplex_positions[i].x + simplex_positions[i].y) % 8.0;
        let angle = direction / 4.0 * std::f64::consts::PI;
        out[i].x = angle.cos();
        out[i].y = angle.sin();
    }

    out
}


fn noise_to_greyscale(value: f64) -> u8 {
    (value * 127.5 + 127.5) as u8
}
