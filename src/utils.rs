


pub fn change_extension_to_content_type(extension: &str) -> String {
    let mut content_type = "";
    if extension == "png"{
        content_type = "image/png"
    }else if extension == "jpg" {
        content_type = "image/jpeg"
    }else if extension == "gif"{
        content_type = "image/gif"
    }
    content_type.to_string()
}
