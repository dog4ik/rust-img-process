use std::time::Instant;

use clap::Parser;
use image_process::merge_dir_recursive;
use image_process::merge_folder;
use image_process::merge_folder_concurrent;
use image_process::merge_image;
use image_process::Cli;
use image_process::Commands;
fn main() {
    let start = Instant::now();
    let args = Cli::parse();
    match args.command {
        Commands::Merge { img1, img2, output } => {
            merge_image(&img1, &img2, &output).expect("to merge");
        }
        Commands::MergeFolder {
            path,
            output,
            concurrent,
        } => {
            if concurrent {
                merge_folder_concurrent(&path, output).unwrap();
            } else {
                merge_folder(&path, output).unwrap();
            }
        }
        Commands::MergeFolderRecursive { path, output } => {
            merge_dir_recursive(path, output);
        }
    };
    let duration = start.elapsed();

    println!("Done in {:?}", duration);
}
