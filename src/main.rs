use bmp::{Image, Pixel};
// use rand::prelude::*;


#[derive(Copy, Clone)]
struct Grad(i32, i32);


static IMAGE_WIDTH: u32 = 256;
static IMAGE_HEIGHT: u32 = 256;
static GRID_WIDTH: i32 = 16;
static GRID_HEIGHT: i32 = 16;

static F2: f64 = (1.732050808 - 1.0) / 2.0;
static G2: f64 = (3.0 - 1.732050808) / 6.0;

static GRADIENTS: [Vector; 8] = [
    Vector { x: 1.0, y: 0.0 },
    Vector { x: 1.0, y: 1.0 },
    Vector { x: 0.0, y: 1.0 },
    Vector { x: -1.0, y: 1.0 },
    Vector { x: -1.0, y: 0.0 },
    Vector { x: -1.0, y: -1.0 },
    Vector { x: 0.0, y: -1.0 },
    Vector { x: 1.0, y: -1.0 },
];


#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
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
        // println!("Pixel ({}, {})", pixel_x, pixel_y);
        let grid_position = pixel_to_grid(pixel_x, pixel_y);
        // println!("Grid ({}, {})", grid_position.x, grid_position.y);
        let value = simplex_noise(grid_position);//(grid_position.x, grid_position.y);
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
        let grey = noise_to_greyscale(value);
        image.set_pixel(pixel_x, pixel_y, Pixel::new(grey, grey, grey));
        // println!("");
    }

    // println!("Range: {} - {}", min, max);

    image.save("image.bmp").unwrap();
}


fn pixel_to_grid(pixel_x: u32, pixel_y: u32) -> Vector {
    let grid_x = pixel_x as f64 / IMAGE_WIDTH as f64 * GRID_WIDTH as f64;
    let grid_y = pixel_y as f64 / IMAGE_HEIGHT as f64 * GRID_HEIGHT as f64;

    Vector { x: grid_x, y: grid_y }
}


// fn noise(sample_x: f64, sample_y: f64) -> f64 {
//     let skew_factor = (sample_x + sample_y) * F2;
//     let skewed_vertex_0_x = (sample_x + skew_factor).floor();
//     let skewed_vertex_0_y = (sample_y + skew_factor).floor();
//     let unskew_factor = (skewed_vertex_0_x + skewed_vertex_0_y) * G2;
//     let X0 = skewed_vertex_0_x - unskew_factor;
//     let Y0 = skewed_vertex_0_y - unskew_factor;
//     let x0 = sample_x - X0;
//     let y0 = sample_y - Y0;
//     let i1: f64;
//     let j1: f64;
//     if x0 > y0 {
//         i1 = 1.0;
//         j1 = 0.0;
//     } else {
//         i1 = 0.0;
//         j1 = 1.0;
//     }
//     let x1 = x0 - i1 + G2;
//     let y1 = y0 - j1 + G2;
//     let x2 = x0 - 1.0 + 2.0 * G2;
//     let y2 = y0 - 1.0 + 2.0 * G2;
//     let gi0 = (skewed_vertex_0_x + skewed_vertex_0_y) as usize % 8;
//     let gi1 = (skewed_vertex_0_x + i1 + skewed_vertex_0_y + j1) as usize % 8;
//     let gi2 = (skewed_vertex_0_x + 1.0 + skewed_vertex_0_y + 1.0) as usize % 8;
//     let mut t0 = 0.5 - x0 * x0 - y0 * y0;
//     let n0 = if t0 < 0.0 {
//         0.0
//     } else {
//         t0 *= t0;
//         t0 * t0 * dot(GRADIENTS[gi0], x0, y0)
//     };
//     let mut t1 = 0.5 - x1 * x1 - y1 * y1;
//     let n1 = if t1 < 0.0 {
//         0.0
//     } else {
//         t1 *= t1;
//         t1 * t1 * dot(GRADIENTS[gi1], x1, y1)
//     };
//     let mut t2 = 0.5 - x2 * x2 - y2 * y2;
//     let n2 = if t2 < 0.0 {
//         0.0
//     } else {
//         t2 *= t2;
//         t2 * t2 * dot(GRADIENTS[gi2], x2, y2)
//     };

//     70.0 * (n0 + n1 + n2)
// }


// fn dot(g: Grad, x: f64, y: f64) -> f64 {
//     g.0 as f64 * x + g.1 as f64 * y
// }


fn simplex_noise(sample_position: Vector) -> f64 {
    let skew_factor = (sample_position.x + sample_position.y) * (3f64.sqrt() - 1.0) / 2.0;
    let skewed_sample_position = Vector { x: sample_position.x + skew_factor, y: sample_position.y + skew_factor };

    let simplex_vertex_0 = Vector { x: skewed_sample_position.x.floor(), y: skewed_sample_position.y.floor() };
    let simplex_vertex_1 = if skewed_sample_position.x > skewed_sample_position.y {
        Vector { x: simplex_vertex_0.x + 1.0, y: simplex_vertex_0.y }
    } else {
        Vector { x: simplex_vertex_0.x, y: simplex_vertex_0.y + 1.0 }
    };
    let simplex_vertex_2 = Vector { x: simplex_vertex_0.x + 1.0, y: simplex_vertex_0.y + 1.0 };

    let gradient_0 = GRADIENTS[(simplex_vertex_0.x + simplex_vertex_0.y) as usize % 8];
    let gradient_1 = GRADIENTS[(simplex_vertex_1.x + simplex_vertex_1.y) as usize % 8];
    let gradient_2 = GRADIENTS[(simplex_vertex_2.x + simplex_vertex_2.y) as usize % 8];

    let unskew_factor = (1f64 - 1f64 / 3f64.sqrt()) / 2f64;
    let unskewed_vertex_0 = simplex_vertex_0 - (simplex_vertex_0.x + simplex_vertex_0.y) * unskew_factor;
    let unskewed_vertex_1 = simplex_vertex_1 - (simplex_vertex_1.x + simplex_vertex_1.y) * unskew_factor;
    let unskewed_vertex_2 = simplex_vertex_2 - (simplex_vertex_2.x + simplex_vertex_2.y) * unskew_factor;

    let distance_0 = sample_position - unskewed_vertex_0;
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

    70.0 * (contribution_0 + contribution_1 + contribution_2)
}


fn noise_to_greyscale(value: f64) -> u8 {
    (value * 127.5 + 127.5) as u8
}
