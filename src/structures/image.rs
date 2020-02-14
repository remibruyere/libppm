use super::w_matrix::W_matrix;
use libc::{c_char, c_int};

/// A pixel being is represented here
#[derive(Clone, Copy)]
pub struct Pixel {
    /// red color : 0 - 255
    pub r: u8,
    /// green color : 0 - 255
    pub g: u8,
    /// blue color : 0 - 255
    pub b: u8,
}

/// A image being is represented here
pub struct Image {
    /// Vec of all pixels of the image
    pub pixels: Vec<Pixel>,
    /// Height value of the image
    pub height: usize,
    /// Width value of the image
    pub width: usize,
}

impl Pixel {
    /// Return a pixel with the red, green, blue value
    ///
    /// # Arguments
    ///
    /// * `red` - An unsigned int of 8 bits that represent red value
    /// * `green` - An unsigned int of 8 bits that represent green value
    /// * `blue` - An unsigned int of 8 bits that represent blue value
    ///
    /// # Example
    ///
    /// ```
    /// use structures::image::Pixel;
    /// let pixel = Pixel::new(10 as u8, 10 as u8, 10 as u8);
    /// ```
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

    /// Display the pixel with the following format
    ///
    /// # Result
    ///
    /// ```
    /// "red: 10, green: 10, blue: 10"
    /// ```
    pub fn display(self) -> String {
        format!("red: {}, green: {}, blue: {}", self.r, self.g, self.b)
    }

    /// Invert the pixel color
    fn invert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }

    fn eq(self, other: Pixel) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }

    /// Convert pixel color to grayscale pixel
    fn grayscale(&mut self) {
        let gray_index = (self.r / 3) + (self.g / 3) + (self.b / 3);
        self.r = gray_index;
        self.g = gray_index;
        self.b = gray_index;
    }
}

impl Image {
    /// Return an image
    ///
    /// # Arguments
    ///
    /// * `vec_r` - Vector of red pixel value
    /// * `vec_g` - Vector of green pixel value
    /// * `vec_b` - Vector of blue pixel value
    /// * `width` - Image pixel width
    /// * `height` - Image pixel height
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

    /// Convert image to vector of red, green, blue values
    ///
    /// # Arguments
    ///
    /// * `image` - A slice of Image instance
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

    /// Invert all pixels of the image
    pub fn invert(&mut self) {
        for pixel in &mut self.pixels {
            pixel.invert();
        }
    }

    /// Apply grayscale to all pixel of the image
    pub fn grayscale(&mut self) {
        for pixel in &mut self.pixels {
            pixel.grayscale();
        }
    }

    /// Apply gaussian blur to the image
    ///
    /// # Arguments
    ///
    /// * `matrix` - Instance of the matrix to apply
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

    /// Apply the given matrix on the pixel
    ///
    /// # Arguments
    ///
    /// * `ci` - current index
    /// * `cloned_image` - backup of the initial image
    /// * `matrix` - instance of W_Matrix
    /// * `total_weight` - Total weight of the initial image
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
