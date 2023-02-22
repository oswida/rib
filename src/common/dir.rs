use std::{
    env,
    fmt::Error,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub fn get_app_path(path: &str) -> String {
    let dir = env::current_dir().unwrap();
    let result = dir.canonicalize().unwrap();
    result.join(path).to_str().unwrap().to_string()
}

pub fn create_app_dir(path: &str) -> Result<(), Error> {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_dir();
    if !b {
        fs::create_dir(p).expect("Cannot create dir")
    }
    Ok(())
}

pub fn app_file_exists(path: &str) -> bool {
    let p = &get_app_path(path);
    Path::new(p).is_file()
}

pub fn write_to_file(path: &str, content: &str) -> Result<(), Error> {
    let p = &get_app_path(path);
    let b: bool = Path::new(p).is_file();
    let mut file: File;
    if !b {
        file = File::create(p).expect("Cannot create file");
    } else {
        file = OpenOptions::new()
            .write(true)
            .append(false)
            .open(p)
            .expect("Cannot open file");
    }
    file.write_all(content.as_bytes())
        .expect("Cannot write to file");
    file.sync_all().expect("Cannot sync file");
    Ok(())
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

pub fn init_app_dirs() {
    let app_dirs = ["cs"];
    for d in app_dirs {
        create_app_dir(d);
    }
}
