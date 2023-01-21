use image_process::merge_image;
use image_process::Config;
use std::env;
use std::process;
use std::time::Instant;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(res) => res,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1)
        }
    };
    let start = Instant::now();
    merge_image(&config.first_path, &config.second_path).unwrap();
    let duration = start.elapsed();
    println!("Finished creating image in {:?} from: ", duration);
    println!("{} {}", config.first_path, config.second_path);
}
