use image::io::Reader as ImageReader;
use image::{GenericImage, ImageBuffer, RgbImage};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

pub fn create_image() -> Result<(), image::ImageError> {
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x % 2 == 0 && y % 2 == 0 {
            image::Rgb([255 as u8, 0, 0])
        } else {
            image::Rgb([0, 0, 0])
        }
    });

    match img.save("./test.png") {
        Ok(res) => Ok(res),
        Err(_) => panic!("failed to save image"),
    }
}
pub fn merge_image(first_path: &String, second_path: &String) -> Result<(), image::ImageError> {
    let first_img = ImageReader::open(first_path)?.decode()?.to_rgb8();
    let second_img = ImageReader::open(second_path)?.decode()?.to_rgb8();
    let (first_width, first_height) = first_img.dimensions();
    let (second_width, second_height) = first_img.dimensions();
    let mut result = RgbImage::new(
        first_width + second_width,
        std::cmp::min(first_height, second_height),
    );
    result.copy_from(&first_img, 0, 0).unwrap();
    result.copy_from(&second_img, first_width, 0).unwrap();

    result.save("./result.jpg")
}
