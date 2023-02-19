use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn get_app_path(path: &str) -> String {
    let dir = env::current_dir().unwrap();
    let result = dir.canonicalize().unwrap();
    result.join(path).to_str().unwrap().to_string()
}

pub fn create_app_dir(path: &str) {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_dir();
    if !b {
        fs::create_dir(p).unwrap();
    }
}

pub fn app_file_exists(path: &str) -> bool {
    let p = &get_app_path(path);
    Path::new(p).is_file()
}

pub fn write_to_file(path: &str, content: &str) {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_file();
    let mut file: File;
    if !b {
        file = File::create(p).unwrap();
    } else {
        file = File::open(p).unwrap();
    }
    file.write_all(content.as_bytes()).unwrap();
}

pub fn write_to_binfile(path: &str, content: &[u8]) {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_file();
    let mut file: File;
    if !b {
        file = File::create(p).unwrap();
    } else {
        file = File::open(p).unwrap();
    }
    file.write_all(content).unwrap();
}

pub fn init_app_dirs() {
    let app_dirs = ["cs"];
    for d in app_dirs {
        create_app_dir(d);
    }
}
