use clap::Parser;
use image_process::merge_folder;
use image_process::merge_image;
use image_process::Cli;
use image_process::Commands;
fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Merge { img1, img2, output } => {
            merge_image(&img1, &img2, &output).expect("to merge");
        }
        Commands::MergeFolder { path, output } => {
            merge_folder(&path, output).unwrap();
        }
    };
}
