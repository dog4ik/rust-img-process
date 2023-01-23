use std::cmp;
use std::{fs, path::PathBuf};

use image::{open, GenericImage, ImageBuffer, RgbImage};

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
    match img.save("./created.png") {
        Ok(res) => Ok(res),
        Err(_) => panic!("failed to save image"),
    }
}
pub fn merge_image(
    first_path: &PathBuf,
    second_path: &PathBuf,
    output: &PathBuf,
) -> Result<(), image::ImageError> {
    let first_img = open(first_path).unwrap().to_rgb8();
    let second_img = open(second_path).unwrap().to_rgb8();
    let (first_width, first_height) = first_img.dimensions();
    let (second_width, second_height) = second_img.dimensions();
    let mut result = RgbImage::from_fn(
        first_width + second_width,
        cmp::max(first_height, second_height),
        |_, _| image::Rgb([255, 255, 255]),
    );

    result
        .copy_from(
            &first_img,
            0,
            cmp::max((second_height as i32 - first_height as i32) / 2, 0) as u32,
        )
        .unwrap();
    result
        .copy_from(
            &second_img,
            first_width,
            cmp::max((first_height as i32 - second_height as i32) / 2, 0) as u32,
        )
        .unwrap();
    result.save(output)
}

pub fn merge_folder(folder_path: &PathBuf, output: PathBuf) -> Result<(), &str> {
    if !folder_path.is_dir() {
        return Err("Dir is not dir");
    }
    let mut path_list: Vec<PathBuf> = Vec::new();
    for file in fs::read_dir(folder_path).expect("always to be dir") {
        match file {
            Ok(res) => {
                let file_path = res.path();
                let file_extention = file_path.extension().unwrap().to_str().unwrap();
                if file_extention != "jpg" && file_extention != "png" {
                    continue;
                }
                path_list.push(res.path());
            }
            Err(_) => continue,
        };
    }
    println!("{:?}", path_list);
    for [first_path, second_path] in path_list.array_windows().step_by(2) {
        let mut new_path = output.clone();
        let mut new_name = first_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        new_name.push_str("&");
        new_name.push_str(second_path.file_name().unwrap().to_str().unwrap());
        new_path.push(PathBuf::from(new_name));
        println!("{:?} {:?}", first_path, second_path);
        merge_image(first_path, second_path, &new_path).unwrap();
    }
    Ok(())
}
