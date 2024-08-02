use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn collect_images_path(path: &PathBuf) -> Vec<PathBuf> {
    let iter1 = match fs::read_dir(path) {
        Ok(read_dir) => read_dir
            .map(|entry| entry.unwrap().path())
            .filter(|item| item.is_file() && is_images(item))
            .collect(),
        Err(_) => Vec::new(),
    };

    let iter2 = match fs::read_dir(path) {
        Ok(read_dir) => read_dir
            .map(|entry| entry.unwrap().path())
            .filter(|item| item.is_dir())
            .flat_map(|item| collect_images_path(&item).into_iter())
            .collect(),
        Err(_) => Vec::new(),
    };
    iter1.into_iter().chain(iter2).collect()
}

const DOT_JPG: &str = ".jpg";
const DOT_PNG: &str = ".png";
const DOT_WEBP: &str = ".webp";

pub fn is_images(pathbuf: &Path) -> bool {
    pathbuf.to_str().unwrap().to_lowercase().ends_with(DOT_JPG)
        || pathbuf.to_str().unwrap().to_lowercase().ends_with(DOT_PNG)
        || pathbuf.to_str().unwrap().to_lowercase().ends_with(DOT_WEBP)
}
