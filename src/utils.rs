pub fn change_extension_to_content_type(extension: &str) -> String {
    let content_type;
    match extension {
        "png" => content_type = "image/png",
        "jpg" => content_type = "image/jpeg",
        "gif" => content_type = "image/gif",
        _ => content_type = "",
    }
    content_type.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_png() {
        assert_eq!(
            change_extension_to_content_type("png"),
            "image/png".to_string()
        );
    }

    #[test]
    fn ut_jpg() {
        assert_eq!(
            change_extension_to_content_type("jpg"),
            "image/jpeg".to_string()
        );
    }

    #[test]
    fn ut_gif() {
        assert_eq!(
            change_extension_to_content_type("gif"),
            "image/gif".to_string()
        );
    }

    #[test]
    fn ut_other() {
        assert_eq!(change_extension_to_content_type("txt"), "".to_string());
    }
}
