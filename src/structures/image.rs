use super::w_matrix::W_matrix;
use libc::{c_char, c_int};

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image {
    pub pixels: Vec<Pixel>,
    pub height: usize,
    pub width: usize,
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue,
        }
    }

    fn red(self) -> u8 {
        self.r
    }

    fn green(self) -> u8 {
        self.g
    }

    fn blue(self) -> u8 {
        self.b
    }

    pub fn display(self) -> String {
        format!("red: {}, green: {}, blue: {}", self.r, self.g, self.b)
    }

    fn invert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }

    fn eq(self, other: Pixel) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }

    fn grayscale(&mut self) {
        let gray_index = (self.r / 3) + (self.g / 3) + (self.b / 3);
        self.r = gray_index;
        self.g = gray_index;
        self.b = gray_index;
    }
}

impl Image {
    pub fn new(
        vec_r: Vec<c_int>,
        vec_g: Vec<c_int>,
        vec_b: Vec<c_int>,
        width: usize,
        height: usize,
    ) -> Image {
        let mut vec_pixel: Vec<Pixel> = Vec::new();
        let size: usize = width * height;
        for i in 0..size {
            vec_pixel.push(Pixel::new(vec_r[i] as u8, vec_g[i] as u8, vec_b[i] as u8));
        }

        Image {
            pixels: vec_pixel,
            height: height,
            width: width,
        }
    }

    pub fn to_vector(image: &Image) -> (Vec<c_int>, Vec<c_int>, Vec<c_int>) {
        let mut r: Vec<c_int> = Vec::new();
        let mut g: Vec<c_int> = Vec::new();
        let mut b: Vec<c_int> = Vec::new();
        for pixel in &image.pixels {
            r.push(pixel.r as c_int);
            g.push(pixel.g as c_int);
            b.push(pixel.b as c_int);
        }
        (r, g, b)
    }

    pub fn invert(&mut self) {
        for pixel in &mut self.pixels {
            pixel.invert();
        }
    }
    pub fn grayscale(&mut self) {
        for pixel in &mut self.pixels {
            pixel.grayscale();
        }
    }

    pub fn gaussian_blur(&mut self, matrix: &W_matrix) {
        let clean_cloned_pixels = self.pixels.clone();
        let total_weight = matrix.get_total_weight();
        for i in 0..self.height {
            for j in 0..self.width {
                let current_index = i * self.width + j;
                self.apply_matrix_on_pixel(
                    current_index,
                    &clean_cloned_pixels,
                    &matrix,
                    total_weight,
                );
            }
        }
    }
    fn apply_matrix_on_pixel(
        &mut self,
        ci: usize,
        cloned_image: &Vec<Pixel>,
        matrix: &W_matrix,
        total_weight: u128,
    ) {
        let mut blur_r: u128 = 0;
        let mut blur_g: u128 = 0;
        let mut blur_b: u128 = 0;
        let current_row = ci / self.width;
        let image_size = self.width * self.height;
        let mut matrix_index = 0;

        let half_matrix_size = (matrix.size / 2) as isize;
        let neg_half_matrix_size: isize = half_matrix_size - matrix.size as isize + 1;

        for width_counter in neg_half_matrix_size..half_matrix_size + 1 {
            for j in neg_half_matrix_size..half_matrix_size + 1 {
                let selected: isize = ci as isize + width_counter * self.width as isize + j;
                if selected >= 0
                    && selected < image_size as isize
                    && selected / self.width as isize == current_row as isize + width_counter
                {
                    blur_r += cloned_image[selected as usize].r as u128
                        * matrix.weight[matrix_index] as u128;
                    blur_g += cloned_image[selected as usize].g as u128
                        * matrix.weight[matrix_index] as u128;
                    blur_b += cloned_image[selected as usize].b as u128
                        * matrix.weight[matrix_index] as u128;
                }
                matrix_index += 1;
            }
        }
        self.pixels[ci] = Pixel {
            r: (blur_r / total_weight) as u8,
            g: (blur_g / total_weight) as u8,
            b: (blur_b / total_weight) as u8,
        };
    }
}
