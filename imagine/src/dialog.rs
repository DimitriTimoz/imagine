use druid::{FileDialogOptions, FileSpec};

pub fn open_image_dialog() -> FileDialogOptions {
    FileDialogOptions::new()
        .name_label("Open")
        .title("Open")
        .button_text("Open")
        .allowed_types(vec![
            FileSpec::new("PNG", &["png"]),
            FileSpec::new("JPG", &["jpg", "jpeg"]),
            FileSpec::new("BMP", &["bmp"]),
            FileSpec::new("GIF", &["gif"]),
            FileSpec::new("ICO", &["ico"]),
            FileSpec::new("TIFF", &["tiff"]),

        ])
}
