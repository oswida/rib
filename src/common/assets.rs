use rust_embed::RustEmbed;

use super::dir::write_to_binfile;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Asset;

// pub fn get_file(path: &str) {
//     let file = Asset::get(path).unwrap();
//     println!("{:?}", std::str::from_utf8(index_html.data.as_ref()));

//     for file in Asset::iter() {
//         println!("{}", file.as_ref());
//     }
// }

pub fn copy_asset(path: &str, destpath: &str) {
    let file = Asset::get(path).unwrap();
    let bytes = file.data.as_ref();
    write_to_binfile(destpath, bytes);
}
