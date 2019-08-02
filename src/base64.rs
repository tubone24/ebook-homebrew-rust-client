extern crate rustc_serialize;

use std::fs::File;
use rustc_serialize::base64::{ToBase64, MIME};
use std::io::Read;
use std::string::String;
use std::fs;

pub fn create_images_b64(directory: &str, content_type: &str, extension: &str) -> Vec<String> {
    let mut images_b64: Vec<String> = Vec::new();
    let paths = fs::read_dir(directory).unwrap();
    for path in paths {
        let filename = path.unwrap().path().to_string_lossy().to_string();
        if !filename.ends_with(extension){
            println!("SKIP: {}", &filename);
            continue;
        }
        println!("ADD: {}", &filename);
        let image_b64 = to_base64(&filename, content_type);
        images_b64.push(image_b64);
    }
    images_b64
}

fn to_base64(path: &str, content_type: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    format!("data:image/{};base64,{}", content_type, base64.replace("\r\n", ""))
}
