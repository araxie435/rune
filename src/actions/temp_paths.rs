use std::path::PathBuf;

pub fn generate_temp_path() -> PathBuf {
    let random_number: u128 = rand::random::<u128>();
    let temp_path:PathBuf  = PathBuf::from(format!("/tmp/rune/{:x}", random_number));
    // NO CHECKS (<1/2^128 chance of collision)
    std::fs::create_dir_all(&temp_path).expect("Failed to create temp directory");

    return temp_path;
}