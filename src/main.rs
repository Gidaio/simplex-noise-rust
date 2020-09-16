use bmp::{Image, Pixel};
use rand::prelude::*;


#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
}

impl std::ops::Add<f64> for Vector {
    type Output = Vector;

    fn add(self, b: f64) -> Vector {
        Vector { x: self.x + b, y: self.y + b }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, b: Vector) -> Vector {
        Vector { x: self.x - b.x, y: self.y - b.y }
    }
}

impl std::ops::Sub<f64> for Vector {
    type Output = Vector;

    fn sub(self, b: f64) -> Vector {
        Vector { x: self.x - b, y: self.y - b }
    }
}

impl std::ops::Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, b: Vector) -> f64 {
        self.x * b.x + self.y * b.y
    }
}


static IMAGE_WIDTH: u32 = 256;
static IMAGE_HEIGHT: u32 = 256;
static GRID_WIDTH: i32 = 16;
static GRID_HEIGHT: i32 = 16;


fn main() {
    let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let gradients = generate_gradients();

    let mut min = 0.0;
    let mut max = 0.0;

    for (pixel_x, pixel_y) in image.coordinates() {
        let grid_position = pixel_to_grid(pixel_x, pixel_y);
        let value = simplex_noise(&gradients, grid_position);

        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }

        let grey = noise_to_greyscale(value);
        image.set_pixel(pixel_x, pixel_y, Pixel::new(grey, grey, grey));
    }

    println!("Min: {}\nMax: {}", min, max);

    image.save("image.bmp").unwrap();
}


fn generate_gradients() -> Vec<Vector> {
    let mut out = vec![];

    for _ in 0..GRID_HEIGHT {
        for _ in 0..GRID_WIDTH {
            let direction = random::<i32>() % 8;
            let angle = direction as f64 * std::f64::consts::FRAC_PI_4;
            out.push(Vector { x: angle.cos(), y: angle.sin() });
        }
    }

    out
}


fn pixel_to_grid(pixel_x: u32, pixel_y: u32) -> Vector {
    let grid_x = pixel_x as f64 / IMAGE_WIDTH as f64 * GRID_WIDTH as f64;
    let grid_y = pixel_y as f64 / IMAGE_HEIGHT as f64 * GRID_HEIGHT as f64;

    Vector { x: grid_x, y: grid_y }
}


fn simplex_noise(gradients: &Vec<Vector>, sample_position: Vector) -> f64 {
    let skew_factor = (sample_position.x + sample_position.y) * (3f64.sqrt() - 1.0) / 2.0;
    let skewed_sample_position = Vector { x: sample_position.x + skew_factor, y: sample_position.y + skew_factor };

    let simplex_vertex_0 = Vector { x: skewed_sample_position.x.floor(), y: skewed_sample_position.y.floor() };
    let unskew_factor = (1.0 - 1.0 / 3f64.sqrt()) / 2.0;
    let unskewed_vertex_0 = simplex_vertex_0 - (simplex_vertex_0.x + simplex_vertex_0.y) * unskew_factor;
    let distance_0 = sample_position - unskewed_vertex_0;

    let simplex_vertex_1 = if distance_0.x > distance_0.y {
        Vector { x: simplex_vertex_0.x + 1.0, y: simplex_vertex_0.y }
    } else {
        Vector { x: simplex_vertex_0.x, y: simplex_vertex_0.y + 1.0 }
    };
    let simplex_vertex_2 = Vector { x: simplex_vertex_0.x + 1.0, y: simplex_vertex_0.y + 1.0 };

    let gradient_0 = gradients[((simplex_vertex_0.x + simplex_vertex_0.y * GRID_HEIGHT as f64) % (GRID_WIDTH * GRID_HEIGHT) as f64) as usize];
    let gradient_1 = gradients[((simplex_vertex_1.x + simplex_vertex_1.y * GRID_HEIGHT as f64) % (GRID_WIDTH * GRID_HEIGHT) as f64) as usize];
    let gradient_2 = gradients[((simplex_vertex_2.x + simplex_vertex_2.y * GRID_HEIGHT as f64) % (GRID_WIDTH * GRID_HEIGHT) as f64) as usize];

    let unskewed_vertex_1 = simplex_vertex_1 - (simplex_vertex_1.x + simplex_vertex_1.y) * unskew_factor;
    let unskewed_vertex_2 = simplex_vertex_2 - (simplex_vertex_2.x + simplex_vertex_2.y) * unskew_factor;

    let distance_1 = sample_position - unskewed_vertex_1;
    let distance_2 = sample_position - unskewed_vertex_2;

    let factor_0 = 0.5 - distance_0 * distance_0;
    let contribution_0 = if factor_0 < 0.0 {
        0.0
    } else {
        factor_0 * factor_0 * factor_0 * factor_0 * (distance_0 * gradient_0)
    };

    let factor_1 = 0.5 - distance_1 * distance_1;
    let contribution_1 = if factor_1 < 0.0 {
        0.0
    } else {
        factor_1 * factor_1 * factor_1 * factor_1 * (distance_1 * gradient_1)
    };

    let factor_2 = 0.5 - distance_2 * distance_2;
    let contribution_2 = if factor_2 < 0.0 {
        0.0
    } else {
        factor_2 * factor_2 * factor_2 * factor_2 * (distance_2 * gradient_2)
    };

    99.0 * (contribution_0 + contribution_1 + contribution_2)
}


fn noise_to_greyscale(value: f64) -> u8 {
    (value * 127.5 + 127.5) as u8
}
