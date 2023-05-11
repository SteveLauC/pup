//! Contains utilities used to deal with target file type.

use std::path::Path;

/// File types that are supported by `pup`
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FileType {
    Markdown,
    Image,
    Unknown,
}

/// Construct a [`FileType`] according to the given `path`
pub fn file_type<P: AsRef<Path>>(path: P) -> FileType {
    match path.as_ref().extension() {
        Some(os_str_type) => {
            if let Some(str_type) = os_str_type.to_str() {
                match str_type {
                    "md" => FileType::Markdown,
                    "jpeg" | "jpg" | "png" | "gif" => FileType::Image,
                    _ => FileType::Unknown,
                }
            } else {
                FileType::Unknown
            }
        }
        None => FileType::Unknown,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image() {
        assert_eq!(FileType::Image, file_type("x.jpeg"));
        assert_eq!(FileType::Image, file_type("x.jpg"));
        assert_eq!(FileType::Image, file_type("x.png"));
        assert_eq!(FileType::Image, file_type("x.gif"));
    }

    #[test]
    fn test_markdown() {
        assert_eq!(FileType::Markdown, file_type("x.md"));
    }

    #[test]
    fn test_unknown() {
        assert_eq!(FileType::Unknown, file_type("x"));
        assert_eq!(FileType::Unknown, file_type("x.weird_format"));
    }
}
