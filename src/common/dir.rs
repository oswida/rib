use std::{
    env,
    fmt::Error,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

/// Get full path to the app data dir.
pub fn get_app_path(path: &str) -> String {
    let dir = env::current_dir().unwrap();
    let result = dir.canonicalize().unwrap();
    result.join(path).to_str().unwrap().to_string()
}

/// Create directory in the app data folder
pub fn create_app_dir(path: &str) -> Result<(), Error> {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_dir();
    if !b {
        fs::create_dir(p).expect("Cannot create dir")
    }
    Ok(())
}

/// Check if file exists in the data directory
pub fn app_file_exists(path: &str) -> bool {
    let p = &get_app_path(path);
    Path::new(p).is_file()
}

/// Write string to file in data directory
pub fn write_to_file(path: &str, content: &str) -> Result<(), Error> {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_file();
    let mut file: File;
    if !b {
        file = File::create(p).expect("Cannot create file");
    } else {
        file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(p)
            .expect("Cannot open file");
    }
    file.write_all(content.as_bytes())
        .expect("Cannot write to file");

    Ok(())
}

/// Delete file from app data dir.
pub fn delete_file(path: &str) -> Result<(), Error> {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_file();
    if !b {
        return Ok(());
    }
    fs::remove_file(p).expect("Cannot delete file");
    Ok(())
}

/// Write binary data to app dir.
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

/// Read file from data directory
pub fn read_from_file(path: &str) -> Result<String, Error> {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_file();
    let mut file: File;
    if !b {
        return Err(Error {});
    } else {
        file = File::open(p).expect("Cannot open file");
    }
    let mut buffer = String::new();
    let result = file.read_to_string(&mut buffer);
    match result {
        Ok(_) => Ok(buffer),
        Err(e) => Err(Error {}),
    }
}

/// Initialize application directories
pub fn init_app_dirs() {
    let app_dirs = ["cs"];
    for d in app_dirs {
        create_app_dir(d);
    }
}
