pub mod structures;
extern crate libc;
use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::str;
use structures::image::Image;
use structures::w_matrix::W_matrix;

//MacOS user need to comment the line below
#[link(name = "ppma_io")]

extern "C" {
    fn ppma_write(
        file_output_name: *const c_char,
        xsize: c_int,
        ysize: c_int,
        r: *const c_int,
        g: *const c_int,
        b: *const c_int,
    );
    fn ppma_read(
        file_input_name: *const c_char,
        xsize: *mut c_int,
        ysize: *mut c_int,
        rgb_max: *mut c_int,
        r: *mut *mut c_int,
        g: *mut *mut c_int,
        b: *mut *mut c_int,
    );
}

/// This function use the function ppma_read of the C library. It make rust arguments compatible for the C function.
///
/// Returns an instance of Image readed from the path in argument.
///
/// # Arguments
///
/// * `path` - The path of an image
fn ppma_read_rust(path: String) -> Image {
    let mut r = Vec::with_capacity(1);
    let mut g = Vec::with_capacity(1);
    let mut b = Vec::with_capacity(1);

    let mut vec_r: Vec<c_int> = Vec::new();
    let mut vec_g: Vec<c_int> = Vec::new();
    let mut vec_b: Vec<c_int> = Vec::new();

    let mut xsize = Vec::with_capacity(1);
    let mut ysize = Vec::with_capacity(1);
    let mut rgb_max = Vec::with_capacity(1);
    let file_input_name = CString::new(path).expect("Failed to create string");

    unsafe {
        ppma_read(
            file_input_name.as_ptr(),
            xsize.as_mut_ptr(),
            ysize.as_mut_ptr(),
            rgb_max.as_mut_ptr(),
            r.as_mut_ptr(),
            g.as_mut_ptr(),
            b.as_mut_ptr(),
        );

        xsize.set_len(1);
        ysize.set_len(1);
        rgb_max.set_len(1);
        r.set_len(1);
        g.set_len(1);
        b.set_len(1);

        let size = xsize[0] * ysize[0];
        let mut r_pointer = r[0];
        let mut g_pointer = g[0];
        let mut b_pointer = b[0];
        for _ in 0..size {
            vec_r.push(*r_pointer);
            vec_g.push(*g_pointer);
            vec_b.push(*b_pointer);

            r_pointer = r_pointer.offset(1);
            g_pointer = g_pointer.offset(1);
            b_pointer = b_pointer.offset(1);
        }

        Image::new(vec_r, vec_g, vec_b, xsize[0] as usize, ysize[0] as usize)
    }
}

/// This function use the function ppma_write of the C library. It make rust arguments compatible for the C function.
///
/// # Arguments
///
/// * `image` - An instance of Image
/// * `file_name` - The path of the image the blur
fn ppma_write_rust(image: Image, file_name: String) {
    let (r, g, b) = Image::to_vector(&image);
    unsafe {
        ppma_write(
            CString::new(file_name)
                .expect("Failed to create string")
                .as_ptr(),
            image.width as c_int,
            image.height as c_int,
            r.as_ptr(),
            g.as_ptr(),
            b.as_ptr(),
        );
    }
}

/// Invert the image in arguments
///
/// # Arguments
///
/// * `name_python` - the path of the image the blur
#[no_mangle]
pub extern "C" fn invert_image(name_python: *const c_char) {
    let bytes = unsafe { CStr::from_ptr(name_python).to_bytes() };
    let name: &str = str::from_utf8(bytes).unwrap();
    let mut img: Image = ppma_read_rust(format!("{}{}", "uploads/", name));
    img.invert();
    ppma_write_rust(img, format!("{}{}", "output/", name));
}

/// Make a grayscale of the image in arguments
///
/// # Arguments
///
/// * `name_python` - the path of the image the blur
#[no_mangle]
pub extern "C" fn grayscale_image(name_python: *const c_char) {
    let bytes = unsafe { CStr::from_ptr(name_python).to_bytes() };
    let name: &str = str::from_utf8(bytes).unwrap();
    let mut img: Image = ppma_read_rust(format!("{}{}", "uploads/", name));
    img.grayscale();
    ppma_write_rust(img, format!("{}{}", "output/", name));
}

/// Blur the given image in arguments
///
/// # Arguments
///
/// * `name_python` - the path of the image the blur
#[no_mangle]
pub extern "C" fn image_gaussian_blur(name_python: *const c_char) {
    let bytes = unsafe { CStr::from_ptr(name_python).to_bytes() };
    let name: &str = str::from_utf8(bytes).unwrap();
    let mut img: Image = ppma_read_rust(format!("{}{}", "uploads/", name));
    //Only accept odd size matrix
    let weight_matrix = W_matrix {
        weight: vec![
            5, 5, 10, 5, 5, 5, 10, 15, 10, 5, 10, 15, 30, 15, 10, 5, 10, 15, 10, 5, 5, 5, 10, 5, 5,
        ],
        size: 5,
    };
    img.gaussian_blur(&weight_matrix);

    ppma_write_rust(img, format!("{}{}", "output/", name));
}
