use std::{fs::File, path::PathBuf};
use zstd::stream::read::Decoder;

pub fn unarchive_to_path(archive_path: &PathBuf, path_to: &PathBuf) {
    let file: File = std::fs::File::open(archive_path).expect("Failed to open archive");
    let decompressor = Decoder::new(file).expect("Failed to create decoder");
    let mut archive = tar::Archive::new(decompressor);
    archive.unpack(path_to).expect("Failed to unpack archive");
}