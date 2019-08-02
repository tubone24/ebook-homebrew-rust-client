extern crate rustc_serialize;

use rustc_serialize::base64::{ToBase64, MIME};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::string::String;

pub fn create_images_b64(directory: &str, content_type: &str, extension: &str) -> Vec<String> {
    let mut images_b64: Vec<String> = Vec::new();
    let paths = fs::read_dir(directory).unwrap();
    for path in paths {
        let filename = path.unwrap().path().to_string_lossy().to_string();
        if !filename.ends_with(extension) {
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
    format!(
        "data:image/{};base64,{}",
        content_type,
        base64.replace("\r\n", "")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_create_images_b64_png() {
        assert_eq!(
            create_images_b64("tests/assets", "image/png", "png"),
            vec![
                to_base64("tests/assets/test_001.png", "image/png"),
                to_base64("tests/assets/test_003mini.png", "image/png")
            ]
        );
    }

    #[test]
    fn ut_create_images_b64_gif() {
        assert_eq!(
            create_images_b64("tests/assets", "image/gif", "gif"),
            vec![to_base64("tests/assets/test_002.gif", "image/gif")]
        );
    }

    #[test]
    fn ut_to_base64() {
        assert_eq!(to_base64("tests/assets/test_003mini.png", "image/png"), "data:image/image/png;base64,iVBORw0KGgoAAAANSUh\
        EUgAAAAoAAAAKCAYAAACNMs+9AAAACXBIWXMAAA7EAAAOxAGVKw4bAAAAm0lEQVQYlY3QPYoCURBF4e9JB2IgzGAgjDswE+PJJnmhW3EBxoIbGFyFSQWmioEuxVnABNJt0u0fCH2i\
        Cg63qm7yQM65jxnGOGMVESWkWhhijhFO+EIPXawj4lDUYRN84g8f2GCPaUppAI0IvxFxfDnlNne0pBHLtmLrxHcUVVWVPD9TQM65g2/84IIl9x77WOC/3rLDNiIuTcoVe4oiZUeUmgwAAAAASUVORK5CYII=");
    }
}
