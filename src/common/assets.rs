use rust_embed::RustEmbed;

use super::dir::write_to_binfile;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Asset;

pub fn copy_asset(path: &str, destpath: &str) {
    let file = Asset::get(path).unwrap();
    let bytes = file.data.as_ref();
    write_to_binfile(destpath, bytes);
}

pub fn get_asset_bytes(path: &str) -> Vec<u8> {
    let file = Asset::get(path).unwrap();
    let bytes = file.data.as_ref().to_vec();
    bytes
}

pub fn get_asset_string(path: &str) -> String {
    let res = String::from_utf8(get_asset_bytes(path));
    match res {
        Ok(response) => response,
        Err(_) => "".to_string(),
    }
}
