//! Local image serving rules for documents' relative images.

use std::path::Path;

/// MIME type for a path the image protocol may serve, or `None` if the
/// extension is not an image. Only images are served, so a document can't use
/// the protocol to read other local files.
pub fn image_mime(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    Some(match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "svg" => "image/svg+xml",
        "bmp" => "image/bmp",
        "ico" => "image/x-icon",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serves_images_only() {
        assert_eq!(image_mime(Path::new("/d/a.PNG")), Some("image/png"));
        assert_eq!(image_mime(Path::new("C:/d/b.jpeg")), Some("image/jpeg"));
        assert_eq!(image_mime(Path::new("/d/c.svg")), Some("image/svg+xml"));
        for p in ["/etc/passwd", "/d/notes.md", "/home/u/.ssh/id_ed25519", "/d/x.png.txt", "/d/png"] {
            assert_eq!(image_mime(Path::new(p)), None, "{p}");
        }
    }
}
