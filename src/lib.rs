#![feature(array_windows)]
mod config_parse;
mod merge;

pub use config_parse::Cli;
pub use config_parse::Commands;
pub use merge::create_image;
pub use merge::merge_folder;
pub use merge::merge_image;
